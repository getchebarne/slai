pub mod process_effect_block_gain;
pub mod process_effect_block_set;
pub mod process_effect_calculated_gamble;
pub mod process_effect_card_add_to_deck;
pub mod process_effect_card_add_to_discard;
pub mod process_effect_card_add_to_hand;
pub mod process_effect_card_discard;
pub mod process_effect_card_discover_select;
pub mod process_effect_card_draw;
pub mod process_effect_card_exhaust;
pub mod process_effect_card_move_to_discard;
pub mod process_effect_card_play;
pub mod process_effect_card_remove;
pub mod process_effect_card_remove_from_deck;
pub mod process_effect_card_retain;
pub mod process_effect_card_reward_clear;
pub mod process_effect_card_setup_pick;
pub mod process_effect_card_transform_roll;
pub mod process_effect_card_upgrade;
pub mod process_effect_card_upgrade_random_in_deck;
pub mod process_effect_chest_open;
pub mod process_effect_combat_end;
pub mod process_effect_combat_start;
pub mod process_effect_damage_deal;
pub mod process_effect_damage_mind_blast;
pub mod process_effect_damage_physical;
pub mod process_effect_death;
pub mod process_effect_distraction_add;
pub mod process_effect_draw_up_to;
pub mod process_effect_energy_gain;
pub mod process_effect_energy_loss;
pub mod process_effect_escape_monster;
pub mod process_effect_escape_plan_check;
pub mod process_effect_event_advance_state;
pub mod process_effect_event_end;
pub mod process_effect_finisher_damage;
pub mod process_effect_flechettes_damage;
pub mod process_effect_glass_knife_decay;
pub mod process_effect_gold_gain;
pub mod process_effect_gold_loss;
pub mod process_effect_gold_steal;
pub mod process_effect_health_gain;
pub mod process_effect_health_gain_pct;
pub mod process_effect_health_loss;
pub mod process_effect_health_loss_pct;
pub mod process_effect_heel_hook_proc;
pub mod process_effect_hexaghost_burn_increase;
pub mod process_effect_hexaghost_divider;
pub mod process_effect_id_card_nightmare_pick;
pub mod process_effect_id_card_nightmare_spawn;
pub mod process_effect_max_health_gain;
pub mod process_effect_max_health_loss;
pub mod process_effect_max_health_loss_pct;
pub mod process_effect_modifier_gain;
pub mod process_effect_modifier_multiply;
pub mod process_effect_modifier_remove;
pub mod process_effect_modifier_set_not_new;
pub mod process_effect_modifier_tick;
pub mod process_effect_monster_spawn;
pub mod process_effect_move_execute;
pub mod process_effect_move_update;
pub mod process_effect_poison_tick;
pub mod process_effect_potion_add_random;
pub mod process_effect_potion_use;
pub mod process_effect_relic_grant_random;
pub mod process_effect_relic_grant_specific;
pub mod process_effect_rest_site_exit;
pub mod process_effect_reward_roll_chest;
pub mod process_effect_reward_roll_combat;
pub mod process_effect_reward_skip;
pub mod process_effect_reward_take_gold;
pub mod process_effect_reward_take_potion;
pub mod process_effect_reward_take_relic;
pub mod process_effect_roll_d100_branch;
pub mod process_effect_room_enter;
pub mod process_effect_set_cost_override;
pub mod process_effect_shuffle_discard_pile_into_draw_pile;
pub mod process_effect_sneaky_strike_proc;
pub mod process_effect_storm_of_steel_proc;
pub mod process_effect_target_clear;
pub mod process_effect_target_set;
pub mod process_effect_turn_end;
pub mod process_effect_turn_start;
pub mod process_effect_unload_discard;

use std::collections::VecDeque;

use rand::Rng;

use crate::consts::MAP_HEIGHT;
use crate::consts::MAP_WIDTH;
use crate::consts::MAX_MONSTERS;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::effect::ZERO_EFFECT;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::game::Location;
use crate::map::has_edge;
use crate::types::CardColor;
use crate::types::CombatState;
use crate::types::Context;
use crate::utils::fill_alive_monster_ids;
use crate::utils::shuffle;

// Stack-allocated buffer for effects that a handler wants to push to the
// front of the effect_queue in order. Build up effects normally, then call
// `push_all_front` once at the end of the handler
pub const MAX_EFFECTS_PER_HANDLER: usize = 32;

pub struct EffectBuf {
    pub effects: [Effect; MAX_EFFECTS_PER_HANDLER],
    pub len: usize,
}

impl EffectBuf {
    pub const fn new() -> Self {
        Self {
            effects: [ZERO_EFFECT; MAX_EFFECTS_PER_HANDLER],
            len: 0,
        }
    }

    pub fn push(&mut self, e: Effect) {
        assert!(self.len < MAX_EFFECTS_PER_HANDLER, "EffectBuf overflow");
        self.effects[self.len] = e;
        self.len += 1;
    }

    pub fn push_all_front(&self, effect_queue: &mut VecDeque<Effect>) {
        for e in self.effects[..self.len].iter().rev() {
            effect_queue.push_front(*e);
        }
    }
}

// Stack-allocated buffer for candidate ids during resolution. Sized for
// CandidatePool::DeckFiltered worst case (full deck), with headroom
pub const MAX_CANDIDATES: usize = 128;

pub struct CandidateBuf {
    pub ids: [usize; MAX_CANDIDATES],
    pub len: usize,
}

impl CandidateBuf {
    pub fn new() -> Self {
        Self {
            ids: [0; MAX_CANDIDATES],
            len: 0,
        }
    }

    pub fn push(&mut self, id: usize) {
        assert!(self.len < MAX_CANDIDATES, "CandidateBuf overflow");
        self.ids[self.len] = id;
        self.len += 1;
    }

    pub fn as_slice(&self) -> &[usize] {
        &self.ids[..self.len]
    }

    pub fn as_mut_slice(&mut self) -> &mut [usize] {
        &mut self.ids[..self.len]
    }

    pub fn truncate(&mut self, n: usize) {
        if n < self.len {
            self.len = n;
        }
    }

    pub fn extend_from_slice(&mut self, src: &[usize]) {
        for &id in src {
            self.push(id);
        }
    }
}

pub enum TargetResolution {
    Resolved,
    AwaitInput { num: u16 },
}

// Iterate in reverse so push_front yields `ids` in original queue order
pub(crate) fn enqueue_direct_targets(
    queue: &mut std::collections::VecDeque<Effect>,
    kind: EffectKind,
    id_source: Option<usize>,
    ids: &[usize],
) {
    for &id in ids.iter().rev() {
        queue.push_front(Effect {
            kind,
            id_source,
            target: Target::Direct(Some(id)),
        });
    }
}

fn resolve_candidates(
    candidate_pool: CandidatePool,
    id_source: usize,
    id_character: usize,
    id_hand: &[usize],
    id_monster_picked: Option<usize>,
    id_alive_monsters: &[usize],
    id_rooms: &[[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    location: Location,
    entities: &[Entity],
    id_pick: &[usize],
    id_deck: &[usize],
    buf_cands: &mut CandidateBuf,
) {
    match candidate_pool {
        CandidatePool::Hand => buf_cands.extend_from_slice(id_hand),
        CandidatePool::MonsterPicked => buf_cands.push(
            id_monster_picked.expect("MonsterPicked pool requires id_monster_picked to be Some"),
        ),
        CandidatePool::Character => buf_cands.push(id_character),
        CandidatePool::Monsters => buf_cands.extend_from_slice(id_alive_monsters),
        CandidatePool::OtherMonsters => {
            for &id in id_alive_monsters {
                if id != id_source {
                    buf_cands.push(id);
                }
            }
        }
        CandidatePool::Source => buf_cands.push(id_source),
        CandidatePool::NextRowRooms => match location {
            Location::Start => {
                for col in 0..MAP_WIDTH {
                    if let Some(id_room) = id_rooms[0][col] {
                        buf_cands.push(id_room);
                    }
                }
            }
            Location::Overworld { y, x } => {
                let y_next = y + 1;
                if y_next >= MAP_HEIGHT {
                    return;
                }
                if let Some(id_current) = id_rooms[y][x] {
                    let current_room = &entities[id_current];
                    for col in 0..MAP_WIDTH {
                        if has_edge(current_room.edges, col) {
                            if let Some(id_room) = id_rooms[y_next][col] {
                                buf_cands.push(id_room);
                            }
                        }
                    }
                }
            }
            Location::BossRoom => {}
        },
        CandidatePool::IdPick => buf_cands.extend_from_slice(id_pick),
        CandidatePool::DeckFiltered(kind) => {
            for &id in id_deck {
                if crate::events::card_in_deck_filter(&entities[id], kind) {
                    buf_cands.push(id);
                }
            }
        }
    }
}

fn resolve_targets(
    candidate_pool: CandidatePool,
    selection: SelectionKind,
    id_source: usize,
    id_character: usize,
    id_hand: &[usize],
    id_monster_picked: Option<usize>,
    id_alive_monsters: &[usize],
    id_rooms: &[[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    location: Location,
    entities: &[Entity],
    id_pick: &[usize],
    id_deck: &[usize],
    rng: &mut impl Rng,
    buf_cands: &mut CandidateBuf,
) -> TargetResolution {
    resolve_candidates(
        candidate_pool,
        id_source,
        id_character,
        id_hand,
        id_monster_picked,
        id_alive_monsters,
        id_rooms,
        location,
        entities,
        id_pick,
        id_deck,
        buf_cands,
    );
    match selection {
        SelectionKind::All => TargetResolution::Resolved,
        SelectionKind::Single => {
            assert_eq!(
                buf_cands.len, 1,
                "SelectionKind::Single resolved to {} candidates",
                buf_cands.len
            );
            TargetResolution::Resolved
        }
        SelectionKind::Random { count } => {
            shuffle(buf_cands.as_mut_slice(), rng);
            buf_cands.truncate(count as usize);
            TargetResolution::Resolved
        }
        SelectionKind::Input { count } => {
            if count as usize >= buf_cands.len {
                TargetResolution::Resolved
            } else {
                TargetResolution::AwaitInput { num: count }
            }
        }
    }
}

// Translate `id_source` from "originating entity" to "actor entity": cards
// delegate up to the character (cards don't carry Strength/Weak/Thorns
// targeting), monsters and the character resolve to themselves. Used by
// damage handlers for source-side scaling and Thorns reflect targeting
pub(crate) fn get_id_actor(entities: &[Entity], id_character: usize, id_source: usize) -> usize {
    if entities[id_source].kind == EntityKind::Card {
        id_character
    } else {
        id_source
    }
}

// Returns true on halt. On AwaitInput, the unresolved Effect is pushed back
// to the queue head; action::handle_action pops it when the player's pick
// consumes it
pub fn process_effect(state: &mut GameState, effect: Effect) -> bool {
    let id_target = match effect.target {
        Target::Direct(t) => t,
        Target::Resolve {
            candidates,
            selection,
        } => {
            let halted =
                resolve_or_halt(state, effect.kind, effect.id_source, candidates, selection);
            if halted {
                state.effect_queue.push_front(effect);
            }
            return halted;
        }
    };

    dispatch_by_kind(state, effect.kind, effect.id_source, id_target);
    false
}

fn resolve_or_halt(
    state: &mut GameState,
    kind: EffectKind,
    id_source: Option<usize>,
    candidates: CandidatePool,
    selection: SelectionKind,
) -> bool {
    // Stack locals
    let mut buf_alive = [0usize; MAX_MONSTERS];
    let mut buf_cands = CandidateBuf::new();

    let alive_n = fill_alive_monster_ids(state, &mut buf_alive);
    let id_source_resolved = id_source.unwrap_or(state.id_character);
    let (id_hand, id_monster_picked, id_pick): (&[usize], Option<usize>, &[usize]) =
        match &state.context {
            Some(Context::Combat(c)) => (&c.id_hand, c.id_monster_picked, &c.id_pick),
            _ => (&[], None, &[]),
        };
    let resolution = resolve_targets(
        candidates,
        selection,
        id_source_resolved,
        state.id_character,
        id_hand,
        id_monster_picked,
        &buf_alive[..alive_n],
        &state.id_rooms,
        state.location,
        &state.entities,
        id_pick,
        &state.id_deck,
        &mut state.rng,
        &mut buf_cands,
    );
    match resolution {
        TargetResolution::Resolved => {
            enqueue_direct_targets(&mut state.effect_queue, kind, id_source, buf_cands.as_slice());
            false
        }
        TargetResolution::AwaitInput { .. } => true,
    }
}

fn dispatch_by_kind(
    state: &mut GameState,
    kind: EffectKind,
    id_source: Option<usize>,
    id_target: Option<usize>,
) {
    match kind {
        EffectKind::CardDraw { count } => {
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            process_effect_card_draw::process_effect_card_draw(
                count,
                &state.entities,
                state.id_character,
                &mut combat.id_pile_draw,
                &mut combat.id_hand,
                &mut combat.id_pile_discard,
                &mut combat.card_last_drawn,
                &mut state.effect_queue,
            )
        }
        EffectKind::DrawUpTo { amount } => {
            let Some(Context::Combat(combat)) = &state.context else {
                unreachable!();
            };
            process_effect_draw_up_to::process_effect_draw_up_to(
                amount,
                &combat.id_hand,
                &mut state.effect_queue,
            )
        }
        EffectKind::CardPlay => {
            // Stack locals
            let mut buf_alive = [0usize; MAX_MONSTERS];
            let alive_n = fill_alive_monster_ids(state, &mut buf_alive);
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            // Snapshot the cost-context counters by-value before the
            // entities mut borrow (Copy types, no borrow conflict)
            let this_turn_discards = combat.this_turn_discards;
            let this_combat_damage_instances_taken = combat.this_combat_damage_instances_taken;
            let energy_current = combat.energy.current;
            process_effect_card_play::process_effect_card_play(
                id_target.unwrap(),
                state.id_character,
                &mut state.entities,
                &buf_alive[..alive_n],
                &mut combat.this_turn_attacks_played,
                this_turn_discards,
                this_combat_damage_instances_taken,
                energy_current,
                &state.id_relics,
                &mut state.effect_queue,
            )
        }
        EffectKind::CardAddToDiscard {
            card_name,
            count,
            upgraded,
        } => {
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            process_effect_card_add_to_discard::process_effect_card_add_to_discard(
                card_name,
                count,
                upgraded,
                &mut state.entities,
                &mut combat.id_pile_discard,
            )
        }
        EffectKind::CardDiscard { source } => {
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            process_effect_card_discard::process_effect_card_discard(
                source,
                id_target.unwrap(),
                &mut state.entities,
                &mut combat.id_hand,
                &mut combat.id_pile_discard,
                &mut combat.this_turn_discards,
                &mut state.effect_queue,
            )
        }
        EffectKind::CardMoveToDiscard => {
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            process_effect_card_move_to_discard::process_effect_card_move_to_discard(
                id_target.unwrap(),
                &mut combat.id_hand,
                &mut combat.id_pile_discard,
            )
        }
        EffectKind::DamageMindBlast => {
            let Some(Context::Combat(combat)) = &state.context else {
                unreachable!();
            };
            process_effect_damage_mind_blast::process_effect_damage_mind_blast(
                id_source,
                id_target.unwrap(),
                combat.id_pile_draw.len(),
                &mut state.effect_queue,
            )
        }
        EffectKind::ShuffleDiscardPileIntoDrawPile => {
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            process_effect_shuffle_discard_pile_into_draw_pile::process_effect_shuffle_discard_pile_into_draw_pile(
                &mut combat.id_pile_draw,
                &mut combat.id_pile_discard,
                &mut state.rng,
            )
        }
        EffectKind::CardRetain => process_effect_card_retain::process_effect_card_retain(
            id_target.unwrap(),
            &mut state.entities,
        ),
        EffectKind::CardSetupPick => {
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            process_effect_card_setup_pick::process_effect_card_setup_pick(
                id_target.unwrap(),
                &mut state.entities,
                &mut combat.id_hand,
                &mut combat.id_pile_draw,
            )
        }
        EffectKind::CardNightmarePick => {
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            process_effect_id_card_nightmare_pick::process_effect_id_card_nightmare_pick(
                &mut state.entities,
                id_target.unwrap(),
                &mut combat.id_card_nightmare,
            )
        }
        EffectKind::CardNightmareSpawn => {
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            process_effect_id_card_nightmare_spawn::process_effect_id_card_nightmare_spawn(
                &mut state.entities,
                &mut combat.id_hand,
                &mut combat.id_pile_discard,
                &mut combat.id_card_nightmare,
            )
        }
        EffectKind::CardExhaust => {
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            process_effect_card_exhaust::process_effect_card_exhaust(
                id_target.unwrap(),
                &mut combat.id_hand,
                &mut combat.id_pile_exhaust,
            )
        }
        EffectKind::CardRemove => {
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            process_effect_card_remove::process_effect_card_remove(
                id_target.unwrap(),
                &mut combat.id_hand,
            )
        }
        EffectKind::CardAddToHand {
            card_name,
            count,
            upgraded,
        } => {
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            process_effect_card_add_to_hand::process_effect_card_add_to_hand(
                card_name,
                count,
                upgraded,
                &mut state.entities,
                &mut combat.id_hand,
                &mut combat.id_pile_discard,
            )
        }
        EffectKind::CalculatedGamble => {
            let Some(Context::Combat(combat)) = &state.context else {
                unreachable!();
            };
            process_effect_calculated_gamble::process_effect_calculated_gamble(
                &combat.id_hand,
                &mut state.effect_queue,
            )
        }
        EffectKind::CardUpgrade => process_effect_card_upgrade::process_effect_card_upgrade(
            id_target.unwrap(),
            &mut state.entities,
        ),
        EffectKind::CardRewardClear => {
            let Some(Context::Reward(reward)) = &mut state.context else {
                unreachable!();
            };
            process_effect_card_reward_clear::process_effect_card_reward_clear(reward);
        }
        EffectKind::RewardRollCombat { room_kind } => {
            // escaped_this_combat is preserved on the Combat context that
            // combat_end leaves behind, until this handler converts it to Reward
            let escaped = match &state.context {
                Some(Context::Combat(c)) => c.escaped_this_combat,
                _ => false,
            };
            let reward = process_effect_reward_roll_combat::process_effect_reward_roll_combat(
                room_kind,
                state.id_character,
                &state.id_relics,
                escaped,
                &mut state.potion_drop_mod,
                &mut state.entities,
                &mut state.rng,
            );
            state.context = Some(Context::Reward(reward));
        }
        EffectKind::RewardRollChest { kind } => {
            let reward = process_effect_reward_roll_chest::process_effect_reward_roll_chest(
                kind,
                &state.id_relics,
                &mut state.entities,
                &mut state.rng,
            );
            state.context = Some(Context::Reward(reward));
        }
        EffectKind::RewardTakePotion => {
            let Some(Context::Reward(reward)) = &mut state.context else {
                unreachable!();
            };
            process_effect_reward_take_potion::process_effect_reward_take_potion(
                reward,
                &mut state.entities,
                state.id_character,
            );
        }
        EffectKind::RewardTakeGold => {
            let Some(Context::Reward(reward)) = &mut state.context else {
                unreachable!();
            };
            process_effect_reward_take_gold::process_effect_reward_take_gold(
                reward,
                &mut state.entities,
                state.id_character,
            );
        }
        EffectKind::RewardSkip => {
            let Some(Context::Reward(reward)) = &mut state.context else {
                unreachable!();
            };
            process_effect_reward_skip::process_effect_reward_skip(reward);
        }
        EffectKind::TargetSet => {
            let id_target = id_target.unwrap();
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            process_effect_target_set::process_effect_target_set(
                &mut combat.id_monster_picked,
                id_target,
            )
        }
        EffectKind::TargetClear => {
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            process_effect_target_clear::process_effect_target_clear(&mut combat.id_monster_picked)
        }
        EffectKind::DamagePhysical { amount } => {
            let id_source = id_source.expect("DamagePhysical requires id_source");
            process_effect_damage_physical::process_effect_damage_physical(
                &state.entities,
                id_source,
                state.id_character,
                id_target.unwrap(),
                amount,
                false,
                &mut state.effect_queue,
            )
        }
        EffectKind::DamagePhysicalIfPoisoned { amount } => {
            let id_source = id_source.expect("DamagePhysicalIfPoisoned requires id_source");
            process_effect_damage_physical::process_effect_damage_physical(
                &state.entities,
                id_source,
                state.id_character,
                id_target.unwrap(),
                amount,
                true, // `if_poisoned`
                &mut state.effect_queue,
            )
        }

        EffectKind::GlassKnifeDecay { delta } => {
            process_effect_glass_knife_decay::process_effect_glass_knife_decay(
                &mut state.entities,
                id_target.unwrap(),
                delta,
            )
        }
        EffectKind::DistractionAdd => {
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            process_effect_distraction_add::process_effect_distraction_add(
                &mut state.entities,
                &mut combat.id_hand,
                &mut combat.id_pile_discard,
                &mut state.rng,
            )
        }
        EffectKind::SetCostOverride { amount } => {
            let id_target = id_target.unwrap();
            let card_cost_override = &mut state.entities[id_target].card_cost_override;
            process_effect_set_cost_override::process_effect_set_cost_override(
                card_cost_override,
                amount,
            )
        }
        EffectKind::EscapePlanCheck { block } => {
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            process_effect_escape_plan_check::process_effect_escape_plan_check(
                &state.entities,
                state.id_character,
                &mut combat.card_last_drawn,
                block,
                &mut state.effect_queue,
            )
        }
        EffectKind::FinisherDamage { damage } => {
            let id_target = id_target.unwrap();
            let Some(Context::Combat(combat)) = &state.context else {
                unreachable!();
            };
            process_effect_finisher_damage::process_effect_finisher_damage(
                combat.this_turn_attacks_played,
                id_source,
                id_target,
                damage,
                &mut state.effect_queue,
            )
        }
        EffectKind::FlechettesDamage { damage } => {
            let id_target = id_target.unwrap();
            let Some(Context::Combat(combat)) = &state.context else {
                unreachable!();
            };
            process_effect_flechettes_damage::process_effect_flechettes_damage(
                &state.entities,
                &combat.id_hand,
                id_source,
                id_target,
                damage,
                &mut state.effect_queue,
            )
        }
        EffectKind::HeelHookProc => {
            let id_target = id_target.unwrap();
            let mods_target = &state.entities[id_target].modifiers;
            process_effect_heel_hook_proc::process_effect_heel_hook_proc(
                mods_target,
                &mut state.effect_queue,
            )
        }
        EffectKind::SneakyStrikeProc { energy } => {
            let Some(Context::Combat(combat)) = &state.context else {
                unreachable!();
            };
            process_effect_sneaky_strike_proc::process_effect_sneaky_strike_proc(
                combat.this_turn_discards,
                energy,
                &mut state.effect_queue,
            )
        }
        EffectKind::StormOfSteelProc { upgraded } => {
            let Some(Context::Combat(combat)) = &state.context else {
                unreachable!();
            };
            process_effect_storm_of_steel_proc::process_effect_storm_of_steel_proc(
                upgraded,
                &combat.id_hand,
                &mut state.effect_queue,
            )
        }
        EffectKind::UnloadDiscard => {
            let Some(Context::Combat(combat)) = &state.context else {
                unreachable!();
            };
            process_effect_unload_discard::process_effect_unload_discard(
                &state.entities,
                &combat.id_hand,
                &mut state.effect_queue,
            )
        }
        EffectKind::DamageDeal { amount } => {
            process_effect_damage_deal::process_effect_damage_deal(
                &mut state.entities,
                id_source,
                state.id_character,
                id_target.unwrap(),
                amount,
                &mut state.effect_queue,
            )
        }
        EffectKind::HealthGain { amount } => {
            let id_target = id_target.unwrap();
            let vitals = &mut state.entities[id_target].vitals;
            process_effect_health_gain::process_effect_health_gain(vitals, amount)
        }
        EffectKind::HealthLoss { amount } => {
            let id_target = id_target.unwrap();
            // MasterfulStab GrowsOnDamageInstanceTaken: bump per character damage
            // event (post-block, so amount > 0 excludes fully-absorbed hits)
            if id_target == state.id_character && amount > 0 {
                if let Some(Context::Combat(combat)) = &mut state.context {
                    combat.this_combat_damage_instances_taken =
                        combat.this_combat_damage_instances_taken.saturating_add(1);
                }
            }
            let entity = &mut state.entities[id_target];
            process_effect_health_loss::process_effect_health_loss(
                entity,
                id_target,
                state.id_character,
                amount,
                &mut state.effect_queue,
            )
        }
        EffectKind::BlockGain { amount } => {
            let id_target = id_target.unwrap();
            let from_card = match id_source {
                Some(id) => state.entities[id].kind == EntityKind::Card,
                None => false,
            };
            let entity = &mut state.entities[id_target];
            process_effect_block_gain::process_effect_block_gain(
                &mut entity.vitals,
                &mut entity.modifiers,
                amount,
                from_card,
            )
        }
        EffectKind::BlockSet { amount } => {
            let id_target = id_target.unwrap();
            let vitals = &mut state.entities[id_target].vitals;
            process_effect_block_set::process_effect_block_set(vitals, amount)
        }
        EffectKind::EnergyGain { amount } => {
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            process_effect_energy_gain::process_effect_energy_gain(&mut combat.energy, amount)
        }
        EffectKind::EnergyLoss { amount } => {
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            process_effect_energy_loss::process_effect_energy_loss(&mut combat.energy, amount)
        }
        EffectKind::ModifierGain { kind, stacks } => {
            let id_target = id_target.unwrap();
            let entity = &mut state.entities[id_target];
            let monster_cycle_count = match entity.kind {
                EntityKind::Monster => Some(entity.monster_cycle_count),
                _ => None,
            };
            process_effect_modifier_gain::process_effect_modifier_gain(
                &mut entity.modifiers,
                kind,
                stacks,
                monster_cycle_count,
            )
        }
        EffectKind::ModifierMultiply { kind, factor } => {
            let id_target = id_target.unwrap();
            let modifiers = &mut state.entities[id_target].modifiers;
            process_effect_modifier_multiply::process_effect_modifier_multiply(
                modifiers, kind, factor,
            )
        }
        EffectKind::ModifierRemove { kind } => {
            let id_target = id_target.unwrap();
            let modifiers = &mut state.entities[id_target].modifiers;
            process_effect_modifier_remove::process_effect_modifier_remove(modifiers, kind)
        }
        EffectKind::ModifierTick => {
            let id_target = id_target.unwrap();
            let modifiers = &mut state.entities[id_target].modifiers;
            process_effect_modifier_tick::process_effect_modifier_tick(modifiers)
        }
        EffectKind::PoisonTick => {
            let id_target = id_target.unwrap();
            let modifiers = &mut state.entities[id_target].modifiers;
            process_effect_poison_tick::process_effect_poison_tick(
                modifiers,
                id_target,
                &mut state.effect_queue,
            )
        }
        EffectKind::ModifierSetNotNew => {
            // Stack locals
            let mut buf_alive = [0usize; MAX_MONSTERS];
            let alive_n = fill_alive_monster_ids(state, &mut buf_alive);
            process_effect_modifier_set_not_new::process_effect_modifier_set_not_new(
                state.id_character,
                &mut state.entities,
                &buf_alive[..alive_n],
            )
        }
        EffectKind::Death => {
            let id_actor = id_target.unwrap();
            // Character can die outside Combat (event damage, rest-site
            // mishaps); the monster list is combat-only
            let empty_monsters: [usize; MAX_MONSTERS] = [0; MAX_MONSTERS];
            let (id_monsters, monster_count): (&[usize; MAX_MONSTERS], u8) = match &state.context {
                Some(Context::Combat(c)) => (&c.id_monsters, c.monster_count),
                _ => (&empty_monsters, 0),
            };
            process_effect_death::process_effect_death(
                id_actor,
                state.id_character,
                id_monsters,
                monster_count,
                &mut state.entities,
                &mut state.effect_queue,
            )
        }
        EffectKind::CombatStart => {
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            process_effect_combat_start::process_effect_combat_start(
                state.id_character,
                &state.id_deck,
                &state.id_relics,
                &mut state.entities,
                &mut combat.id_pile_draw,
                &mut combat.id_hand,
                &mut combat.id_pile_discard,
                &mut combat.id_pile_exhaust,
                &mut combat.id_monster_picked,
                &mut combat.this_combat_damage_instances_taken,
                &mut combat.escaped_this_combat,
                &mut state.rng,
                &mut state.effect_queue,
            )
        }
        EffectKind::CombatEnd => {
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            process_effect_combat_end::process_effect_combat_end(
                state.id_character,
                &mut combat.id_hand,
                &mut combat.id_pile_draw,
                &mut combat.id_pile_discard,
                &mut combat.id_pile_exhaust,
                &mut combat.id_monster_picked,
                &mut state.entities,
                &mut combat.monster_count,
                &mut combat.id_card_nightmare,
                &state.id_rooms,
                state.location,
                &mut state.rng,
                &mut state.effect_queue,
            )
        }
        EffectKind::TurnStart => {
            let id_actor = id_target.unwrap();

            // Stack locals
            let mut buf_alive = [0usize; MAX_MONSTERS];
            let alive_n = fill_alive_monster_ids(state, &mut buf_alive);
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            let nightmare_pending = combat.id_card_nightmare.is_some();
            // Energy ref must outlive the entities borrow below; capture by value
            let energy_snapshot = combat.energy;
            let entity = &mut state.entities[id_actor];
            process_effect_turn_start::process_effect_turn_start(
                &mut entity.vitals,
                &mut entity.modifiers,
                id_actor,
                state.id_character,
                &energy_snapshot,
                &buf_alive[..alive_n],
                nightmare_pending,
                &mut state.effect_queue,
            )
        }
        EffectKind::TurnEnd => {
            let id_actor = id_target.unwrap();
            if id_actor == state.id_character {
                // Stack locals
                let mut buf_alive = [0usize; MAX_MONSTERS];
                let alive_n = fill_alive_monster_ids(state, &mut buf_alive);
                let Some(Context::Combat(combat)) = &mut state.context else {
                    unreachable!("TurnEnd (character) outside Combat context");
                };
                process_effect_turn_end::process_effect_turn_end_character(
                    state.id_character,
                    &mut state.entities,
                    &combat.id_hand,
                    &buf_alive[..alive_n],
                    &mut combat.this_turn_discards,
                    &mut combat.this_turn_attacks_played,
                    &state.id_relics,
                    &mut state.effect_queue,
                )
            } else {
                process_effect_turn_end::process_effect_turn_end_monster(
                    &state.entities[id_actor].modifiers,
                    id_actor,
                    &mut state.effect_queue,
                )
            }
        }
        EffectKind::MoveUpdate => {
            let id_target = id_target.unwrap();
            let ascension_level = state.ascension;
            let mut buf_alive = [0usize; MAX_MONSTERS];
            let alive_n = fill_alive_monster_ids(state, &mut buf_alive);
            let alive_monsters = &buf_alive[..alive_n];
            let entity = &mut state.entities[id_target];
            process_effect_move_update::process_effect_move_update(
                entity,
                id_target,
                alive_monsters,
                ascension_level,
                &mut state.rng,
            )
        }
        EffectKind::MoveExecute => {
            let id_target = id_target.unwrap();
            let entity = &state.entities[id_target];
            process_effect_move_execute::process_effect_move_execute(
                entity,
                id_target,
                state.id_character,
                &mut state.effect_queue,
            )
        }
        EffectKind::RoomEnter => process_effect_room_enter::process_effect_room_enter(
            &state.id_rooms,
            state.location,
            &mut state.entities,
            &mut state.encounter_list_normal,
            &mut state.encounter_list_elite,
            state.encounter_boss,
            &mut state.events_seen_this_run,
            &mut state.context,
            &mut state.rng,
            &mut state.effect_queue,
        ),
        EffectKind::RestSiteExit => process_effect_rest_site_exit::process_effect_rest_site_exit(
            &mut state.location,
            &mut state.effect_queue,
        ),
        EffectKind::MonsterSpawn { name } => {
            // First MonsterSpawn of a combat installs the Combat context
            if !matches!(&state.context, Some(Context::Combat(_))) {
                state.context = Some(Context::Combat(CombatState::new()));
            }
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!()
            };
            process_effect_monster_spawn::process_effect_monster_spawn(
                name,
                id_source,
                state.ascension,
                &mut state.entities,
                &mut combat.id_monsters,
                &mut combat.monster_count,
                &mut state.rng,
                &mut state.effect_queue,
            )
        }
        EffectKind::EscapeMonster => {
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            process_effect_escape_monster::process_effect_escape_monster(
                id_target.unwrap(),
                &combat.id_monsters,
                combat.monster_count,
                &mut state.entities,
                &mut combat.escaped_this_combat,
                &mut state.effect_queue,
            )
        }
        EffectKind::GoldSteal { amount } => process_effect_gold_steal::process_effect_gold_steal(
            id_source.unwrap(),
            state.id_character,
            amount,
            &mut state.entities,
        ),
        EffectKind::HexaghostBurnIncrease { count } => {
            let Some(Context::Combat(combat)) = &state.context else {
                unreachable!();
            };
            process_effect_hexaghost_burn_increase::process_effect_hexaghost_burn_increase(
                count,
                &mut state.entities,
                &combat.id_pile_draw,
                &combat.id_pile_discard,
                &mut state.effect_queue,
            )
        }
        EffectKind::HexaghostDivider => {
            process_effect_hexaghost_divider::process_effect_hexaghost_divider(
                id_source,
                state.id_character,
                &state.entities,
                &mut state.effect_queue,
            )
        }
        EffectKind::GoldGain { amount } => {
            let character = &mut state.entities[state.id_character];
            process_effect_gold_gain::process_effect_gold_gain(character, amount)
        }
        // RoomSelect Direct form (after the resolver picked a target)
        // completes the transition. Before resolution it's handled by the
        // `Resolve` branch in `process_effect`
        EffectKind::RoomSelect => {
            let id_room = id_target.expect("RoomSelect Direct form must have target");
            let room = &state.entities[id_room];
            state.location = Location::Overworld {
                y: room.room_y,
                x: room.room_x,
            };
            state
                .effect_queue
                .push_front(Effect::direct(EffectKind::RoomEnter, None, None));
        }
        EffectKind::RewardTakeRelic => {
            let Some(Context::Reward(reward)) = &mut state.context else {
                unreachable!();
            };
            process_effect_reward_take_relic::process_effect_reward_take_relic(
                reward,
                &state.entities,
                &mut state.id_relics,
            );
        }
        EffectKind::CardRemoveFromDeck => {
            process_effect_card_remove_from_deck::process_effect_card_remove_from_deck(
                id_target.unwrap(),
                &mut state.id_deck,
            )
        }
        EffectKind::CardAddToDeck {
            card_name,
            upgraded,
        } => process_effect_card_add_to_deck::process_effect_card_add_to_deck(
            card_name,
            upgraded,
            &mut state.entities,
            &mut state.id_deck,
        ),
        EffectKind::MaxHealthGain { amount } => {
            let id_target = id_target.unwrap();
            let vitals = &mut state.entities[id_target].vitals;
            process_effect_max_health_gain::process_effect_max_health_gain(vitals, amount)
        }
        EffectKind::MaxHealthLoss { amount } => {
            let id_target = id_target.unwrap();
            let vitals = &mut state.entities[id_target].vitals;
            process_effect_max_health_loss::process_effect_max_health_loss(vitals, amount)
        }
        EffectKind::ChestOpen => process_effect_chest_open::process_effect_chest_open(
            &state.id_rooms,
            state.location,
            &mut state.entities,
            &mut state.effect_queue,
        ),
        EffectKind::PotionUse => process_effect_potion_use::process_effect_potion_use(
            id_target.unwrap(),
            &state.entities,
            &mut state.effect_queue,
        ),
        EffectKind::PotionAddRandom { limited } => {
            process_effect_potion_add_random::process_effect_potion_add_random(
                limited,
                state.id_character,
                &mut state.entities,
                &mut state.rng,
            )
        }
        EffectKind::CardDiscoverSelect { kind, count } => {
            let id_cards = process_effect_card_discover_select::process_effect_card_discover_select(
                kind,
                CardColor::Green, // TODO: other characters
                count,
                &mut state.entities,
                &mut state.rng,
            );
            // Stash rolled candidates on combat.id_pick and queue a
            // CardDiscoverPick halt for the player's pick
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            combat.id_pick = id_cards;
            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardDiscoverPick,
                id_source: None,
                target: Target::Resolve {
                    candidates: CandidatePool::IdPick,
                    selection: SelectionKind::Input { count: 1 },
                },
            });
        }
        EffectKind::GoldLoss { amount } => {
            let character = &mut state.entities[state.id_character];
            process_effect_gold_loss::process_effect_gold_loss(character, amount)
        }
        EffectKind::HealthGainPct { numer, denom } => {
            let character = &state.entities[state.id_character];
            process_effect_health_gain_pct::process_effect_health_gain_pct(
                character,
                state.id_character,
                numer,
                denom,
                &mut state.effect_queue,
            )
        }
        EffectKind::HealthLossPct { numer, denom } => {
            let character = &state.entities[state.id_character];
            process_effect_health_loss_pct::process_effect_health_loss_pct(
                character,
                state.id_character,
                numer,
                denom,
                &mut state.effect_queue,
            )
        }
        EffectKind::MaxHealthLossPct { numer, denom } => {
            let character = &state.entities[state.id_character];
            process_effect_max_health_loss_pct::process_effect_max_health_loss_pct(
                character,
                state.id_character,
                numer,
                denom,
                &mut state.effect_queue,
            )
        }
        EffectKind::CardUpgradeRandomInDeck { count } => {
            process_effect_card_upgrade_random_in_deck::process_effect_card_upgrade_random_in_deck(
                count,
                &state.entities,
                &state.id_deck,
                &mut state.rng,
                &mut state.effect_queue,
            )
        }
        EffectKind::CardTransformRoll => {
            process_effect_card_transform_roll::process_effect_card_transform_roll(
                &mut state.rng,
                &mut state.effect_queue,
            )
        }
        EffectKind::RelicGrantRandom { tier } => {
            process_effect_relic_grant_random::process_effect_relic_grant_random(
                tier,
                &mut state.id_relics,
                &mut state.entities,
                &mut state.rng,
            )
        }
        EffectKind::RelicGrantSpecific {
            name,
            fallback_circlet,
        } => process_effect_relic_grant_specific::process_effect_relic_grant_specific(
            name,
            fallback_circlet,
            &mut state.id_relics,
            &mut state.entities,
        ),
        EffectKind::EventAdvanceState { delta } => {
            let id_event = id_source.expect("EventAdvanceState requires id_source");
            process_effect_event_advance_state::process_effect_event_advance_state(
                &mut state.entities,
                id_event,
                delta,
            )
        }
        EffectKind::RollD100Branch {
            chance,
            on_lt,
            on_ge,
        } => process_effect_roll_d100_branch::process_effect_roll_d100_branch(
            chance,
            on_lt,
            on_ge,
            id_source,
            &mut state.rng,
            &mut state.effect_queue,
        ),
        EffectKind::EventEnd => {
            let id_event = id_source.expect("EventEnd requires id_source");
            process_effect_event_end::process_effect_event_end(&mut state.entities, id_event)
        }
        EffectKind::DeckSelectStart { kind } => {
            // Push a DeckSelectPick halt at the queue front. The resolver
            // computes candidates via CandidatePool::DeckFiltered(kind); if
            // the filter yields an empty set, resolve_targets short-circuits
            // and the pick is skipped
            state.effect_queue.push_front(Effect {
                kind: EffectKind::DeckSelectPick { kind },
                id_source: None,
                target: Target::Resolve {
                    candidates: CandidatePool::DeckFiltered(kind),
                    selection: SelectionKind::Input { count: 1 },
                },
            });
        }
        EffectKind::CardDiscoverPick => {
            let id_card = id_target.expect("CardDiscoverPick Direct form must have target");
            let Some(Context::Combat(combat)) = &mut state.context else {
                unreachable!();
            };
            state.entities[id_card].card_free_to_play_once = true;
            combat.id_hand.push(id_card);
            combat.id_pick.clear();
        }
        EffectKind::DeckSelectPick { kind: ds_kind } => {
            let id_card = id_target.expect("DeckSelectPick Direct form must have target");
            // Apply the deck-pick action by kind (same as today's
            // deck_select_follow_up in action.rs). Push effects to queue front
            match ds_kind {
                crate::types::DeckSelectKind::Remove => {
                    state.effect_queue.push_front(Effect::direct(
                        EffectKind::CardRemoveFromDeck,
                        None,
                        Some(id_card),
                    ));
                }
                crate::types::DeckSelectKind::UpgradeAny => {
                    state.effect_queue.push_front(Effect::direct(
                        EffectKind::CardUpgrade,
                        None,
                        Some(id_card),
                    ));
                }
                crate::types::DeckSelectKind::DuplicateAny => {
                    let card = &state.entities[id_card];
                    let card_name = card.card_name;
                    let upgraded = card.card_upgraded;
                    state.effect_queue.push_front(Effect::direct(
                        EffectKind::CardAddToDeck {
                            card_name,
                            upgraded,
                        },
                        None,
                        None,
                    ));
                }
                crate::types::DeckSelectKind::TransformOne => {
                    state.effect_queue.push_front(Effect::direct(
                        EffectKind::CardTransformRoll,
                        None,
                        None,
                    ));
                    state.effect_queue.push_front(Effect::direct(
                        EffectKind::CardRemoveFromDeck,
                        None,
                        Some(id_card),
                    ));
                }
            }
        }
        EffectKind::Noop => panic!("Noop effect should never be dispatched"),
    }
}

pub fn process_queue(state: &mut GameState) {
    while !state.entities[state.id_character].dead {
        let Some(effect) = state.effect_queue.pop_front() else {
            return;
        };
        if process_effect(state, effect) {
            return;
        }
        clear_drained_reward_context(state);
        clear_consumed_event_context(state);
    }
}

// Clear Reward context once the pool is drained (handlers drain in-place via
// reward.id_cards/id_relic/.../gold) and queue a RoomSelect halt to return
// the player to the Map screen
fn clear_drained_reward_context(state: &mut GameState) {
    let drained = match &state.context {
        Some(Context::Reward(r)) => {
            r.id_cards.is_empty()
                && r.id_relic.is_none()
                && r.id_potion.is_none()
                && r.gold.is_none()
        }
        _ => false,
    };
    if drained {
        state.context = None;
        queue_room_select(state);
    }
}

// Clear Event context once the event is consumed (set by EventEnd handler on
// the event entity) and queue a RoomSelect halt
fn clear_consumed_event_context(state: &mut GameState) {
    let consumed = match &state.context {
        Some(Context::Event(e)) => state.entities[e.id_event].event_consumed,
        _ => false,
    };
    if consumed {
        state.context = None;
        queue_room_select(state);
    }
}

fn queue_room_select(state: &mut GameState) {
    state.effect_queue.push_back(Effect {
        kind: EffectKind::RoomSelect,
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::NextRowRooms,
            selection: SelectionKind::Input { count: 1 },
        },
    });
}
