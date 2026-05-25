pub mod process_effect_block_gain;
pub mod process_effect_block_set;
pub mod process_effect_calculated_gamble;
pub mod process_effect_card_add_to_deck;
pub mod process_effect_card_add_to_discard;
pub mod process_effect_card_add_to_hand;
pub mod process_effect_card_discard;
pub mod process_effect_card_discover_pick;
pub mod process_effect_card_discover_select;
pub mod process_effect_card_draw;
pub mod process_effect_card_exhaust;
pub mod process_effect_card_move_to_discard;
pub mod process_effect_card_play;
pub mod process_effect_card_remove;
pub mod process_effect_card_remove_from_deck;
pub mod process_effect_card_retain;
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
pub mod process_effect_deck_select_pick;
pub mod process_effect_deck_select_start;
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
pub mod process_effect_reward_take;
pub mod process_effect_roll_d100_branch;
pub mod process_effect_room_enter;
pub mod process_effect_room_select;
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
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::game::Location;
use crate::map::has_edge;
use crate::types::Screen;
use crate::types::CardColor;
use crate::utils::shuffle;

// Drain state.buf_effects back-to-front into the effect_queue front, so the
// effects pop in the order they were pushed into buf_effects
pub fn flush_effects_from_buf_to_queue_front(state: &mut GameState) {
    while let Some(e) = state.buf_effects.pop() {
        state.effect_queue.push_front(e);
    }
}

pub enum TargetResolution {
    Resolved,
    AwaitInput { num: u16 },
}

// Append an Entity to the arena; returns the assigned id
pub fn push_entity(entities: &mut Vec<Entity>, e: Entity) -> usize {
    let id = entities.len();
    entities.push(e);
    id
}

// Iterate in reverse so push_front yields `ids` in original queue order
pub(crate) fn enqueue_direct_targets(
    id_source: Option<usize>,
    id_targets: &[usize],
    kind: EffectKind,
    queue: &mut VecDeque<Effect>,
) {
    for &id_target in id_targets.iter().rev() {
        queue.push_front(Effect {
            kind,
            id_source,
            target: Target::Direct(Some(id_target)),
        });
    }
}

fn resolve_candidates(
    candidate_pool: CandidatePool,
    id_source: usize,
    id_character: usize,
    id_hand: &[usize],
    id_monster_picked: Option<usize>,
    id_monsters: &[Option<usize>; MAX_MONSTERS],
    id_rooms: &[[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    location: Location,
    entities: &[Entity],
    id_pick: &[usize],
    id_deck: &[usize],
    buf_cands: &mut Vec<usize>,
) {
    match candidate_pool {
        CandidatePool::Hand => buf_cands.extend_from_slice(id_hand),
        CandidatePool::MonsterPicked => buf_cands.push(
            id_monster_picked.expect("MonsterPicked pool requires id_monster_picked to be Some"),
        ),
        CandidatePool::Character => buf_cands.push(id_character),
        CandidatePool::Monsters => buf_cands.extend(id_monsters.iter().flatten().copied()),
        CandidatePool::OtherMonsters => {
            for id in id_monsters.iter().flatten().copied() {
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
    selection_kind: SelectionKind,
    id_source: usize,
    id_character: usize,
    id_hand: &[usize],
    id_monster_picked: Option<usize>,
    id_monsters: &[Option<usize>; MAX_MONSTERS],
    id_rooms: &[[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    location: Location,
    entities: &[Entity],
    id_pick: &[usize],
    id_deck: &[usize],
    rng: &mut impl Rng,
    buf_cands: &mut Vec<usize>,
) -> TargetResolution {
    resolve_candidates(
        candidate_pool,
        id_source,
        id_character,
        id_hand,
        id_monster_picked,
        id_monsters,
        id_rooms,
        location,
        entities,
        id_pick,
        id_deck,
        buf_cands,
    );
    match selection_kind {
        SelectionKind::All => TargetResolution::Resolved,
        SelectionKind::Single => {
            assert_eq!(
                buf_cands.len(),
                1,
                "SelectionKind::Single resolved to {} candidates",
                buf_cands.len()
            );
            TargetResolution::Resolved
        }
        SelectionKind::Random { count } => {
            shuffle(buf_cands.as_mut_slice(), rng);
            buf_cands.truncate(count as usize);
            TargetResolution::Resolved
        }
        SelectionKind::Input { count } => {
            if count as usize >= buf_cands.len() {
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

// Returns true on halt. On AwaitInput, sets state.modal so the action handler
// for the player's pick can build the resolved effect(s) without re-reading
// the queue. Modal is cleared by that handler
pub fn process_effect(state: &mut GameState, effect: Effect) -> bool {
    let id_target = match effect.target {
        Target::Direct(id_target) => id_target,
        Target::Resolve {
            candidate_pool,
            selection_kind,
        } => {
            let halted = resolve_or_halt(
                state,
                effect.kind,
                effect.id_source,
                candidate_pool,
                selection_kind,
            );
            if halted {
                state.pending_effect = Some(effect);
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
    candidate_pool: CandidatePool,
    selection_kind: SelectionKind,
) -> bool {
    let id_source_resolved = id_source.unwrap_or(state.id_character);
    let (id_hand, id_monster_picked, id_pick): (&[usize], Option<usize>, &[usize]) =
        if matches!(state.active, Screen::Combat) {
            (&state.id_hand, state.id_monster_picked, &state.id_pick)
        } else {
            (&[], None, &[])
        };
    state.buf_candidates.clear();
    let resolution = resolve_targets(
        candidate_pool,
        selection_kind,
        id_source_resolved,
        state.id_character,
        id_hand,
        id_monster_picked,
        &state.id_monsters,
        &state.id_rooms,
        state.location,
        &state.entities,
        id_pick,
        &state.id_deck,
        &mut state.rng,
        &mut state.buf_candidates,
    );
    match resolution {
        TargetResolution::Resolved => {
            enqueue_direct_targets(
                id_source,
                &state.buf_candidates,
                kind,
                &mut state.effect_queue,
            );
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
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_card_draw::process_effect_card_draw(state, count)
        }
        EffectKind::DrawUpTo { amount } => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_draw_up_to::process_effect_draw_up_to(state, amount)
        }
        EffectKind::CardPlay => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_card_play::process_effect_card_play(id_target, state)
        }
        EffectKind::CardAddToDiscard {
            card_name,
            count,
            upgraded,
        } => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_card_add_to_discard::process_effect_card_add_to_discard(
                state, card_name, count, upgraded,
            )
        }
        EffectKind::CardDiscard { source } => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_card_discard::process_effect_card_discard(id_target, state, source)
        }
        EffectKind::CardMoveToDiscard => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_card_move_to_discard::process_effect_card_move_to_discard(
                id_target, state,
            )
        }
        EffectKind::DamageMindBlast => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_damage_mind_blast::process_effect_damage_mind_blast(
                id_source, id_target, state,
            )
        }
        EffectKind::ShuffleDiscardPileIntoDrawPile => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_shuffle_discard_pile_into_draw_pile::process_effect_shuffle_discard_pile_into_draw_pile(state)
        }
        EffectKind::CardRetain => {
            process_effect_card_retain::process_effect_card_retain(id_target, state)
        }
        EffectKind::CardSetupPick => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_card_setup_pick::process_effect_card_setup_pick(id_target, state)
        }
        EffectKind::CardNightmarePick => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_id_card_nightmare_pick::process_effect_id_card_nightmare_pick(
                id_target, state,
            )
        }
        EffectKind::CardNightmareSpawn => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_id_card_nightmare_spawn::process_effect_id_card_nightmare_spawn(state)
        }
        EffectKind::CardExhaust => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_card_exhaust::process_effect_card_exhaust(id_target, state)
        }
        EffectKind::CardRemove => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_card_remove::process_effect_card_remove(id_target, state)
        }
        EffectKind::CardAddToHand {
            card_name,
            count,
            upgraded,
        } => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_card_add_to_hand::process_effect_card_add_to_hand(
                state, card_name, count, upgraded,
            )
        }
        EffectKind::CalculatedGamble => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_calculated_gamble::process_effect_calculated_gamble(state)
        }
        EffectKind::CardUpgrade => {
            process_effect_card_upgrade::process_effect_card_upgrade(id_target, state)
        }
        EffectKind::RewardRollCombat { room_kind } => {
            process_effect_reward_roll_combat::process_effect_reward_roll_combat(state, room_kind);
        }
        EffectKind::RewardRollChest { kind } => {
            process_effect_reward_roll_chest::process_effect_reward_roll_chest(state, kind);
        }
        EffectKind::RewardTake { kind } => {
            debug_assert!(matches!(state.active, Screen::Reward));
            process_effect_reward_take::process_effect_reward_take(id_target, state, kind)
        }
        EffectKind::RewardSkip => {
            debug_assert!(matches!(state.active, Screen::Reward));
            process_effect_reward_skip::process_effect_reward_skip(state);
        }
        EffectKind::TargetSet => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_target_set::process_effect_target_set(id_target, state)
        }
        EffectKind::TargetClear => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_target_clear::process_effect_target_clear(state)
        }
        EffectKind::DamagePhysical { amount } => {
            process_effect_damage_physical::process_effect_damage_physical(
                id_source, id_target, state, amount, false,
            )
        }
        EffectKind::DamagePhysicalIfPoisoned { amount } => {
            process_effect_damage_physical::process_effect_damage_physical(
                id_source, id_target, state, amount, true,
            )
        }
        EffectKind::GlassKnifeDecay { delta } => {
            process_effect_glass_knife_decay::process_effect_glass_knife_decay(
                id_target, state, delta,
            )
        }
        EffectKind::DistractionAdd => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_distraction_add::process_effect_distraction_add(state)
        }
        EffectKind::SetCostOverride { amount } => {
            process_effect_set_cost_override::process_effect_set_cost_override(
                id_target, state, amount,
            )
        }
        EffectKind::EscapePlanCheck { block } => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_escape_plan_check::process_effect_escape_plan_check(state, block)
        }
        EffectKind::FinisherDamage { damage } => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_finisher_damage::process_effect_finisher_damage(
                id_source, id_target, state, damage,
            )
        }
        EffectKind::FlechettesDamage { damage } => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_flechettes_damage::process_effect_flechettes_damage(
                id_source, id_target, state, damage,
            )
        }
        EffectKind::HeelHookProc => {
            process_effect_heel_hook_proc::process_effect_heel_hook_proc(id_target, state)
        }
        EffectKind::SneakyStrikeProc { energy } => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_sneaky_strike_proc::process_effect_sneaky_strike_proc(state, energy)
        }
        EffectKind::StormOfSteelProc { upgraded } => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_storm_of_steel_proc::process_effect_storm_of_steel_proc(state, upgraded)
        }
        EffectKind::UnloadDiscard => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_unload_discard::process_effect_unload_discard(state)
        }
        EffectKind::DamageDeal { amount } => {
            process_effect_damage_deal::process_effect_damage_deal(
                id_source, id_target, state, amount,
            )
        }
        EffectKind::HealthGain { amount } => {
            process_effect_health_gain::process_effect_health_gain(id_target, state, amount)
        }
        EffectKind::HealthLoss { amount } => {
            process_effect_health_loss::process_effect_health_loss(id_target, state, amount)
        }
        EffectKind::BlockGain { amount } => process_effect_block_gain::process_effect_block_gain(
            id_source, id_target, state, amount,
        ),
        EffectKind::BlockSet { amount } => {
            process_effect_block_set::process_effect_block_set(id_target, state, amount)
        }
        EffectKind::EnergyGain { amount } => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_energy_gain::process_effect_energy_gain(state, amount)
        }
        EffectKind::EnergyLoss { amount } => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_energy_loss::process_effect_energy_loss(state, amount)
        }
        EffectKind::ModifierGain { kind, stacks } => {
            process_effect_modifier_gain::process_effect_modifier_gain(
                id_target, state, kind, stacks,
            )
        }
        EffectKind::ModifierMultiply { kind, factor } => {
            process_effect_modifier_multiply::process_effect_modifier_multiply(
                id_target, state, kind, factor,
            )
        }
        EffectKind::ModifierRemove { kind } => {
            process_effect_modifier_remove::process_effect_modifier_remove(id_target, state, kind)
        }
        EffectKind::ModifierTick => {
            process_effect_modifier_tick::process_effect_modifier_tick(id_target, state)
        }
        EffectKind::PoisonTick => {
            process_effect_poison_tick::process_effect_poison_tick(id_target, state)
        }
        EffectKind::ModifierSetNotNew => {
            process_effect_modifier_set_not_new::process_effect_modifier_set_not_new(state)
        }
        EffectKind::Death => {
            // Character can die outside Combat (event damage, rest-site mishaps);
            // monster slots are then empty so the iter-skip-Nones is a no-op
            process_effect_death::process_effect_death(id_target, state)
        }
        EffectKind::CombatStart => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_combat_start::process_effect_combat_start(state)
        }
        EffectKind::CombatEnd => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_combat_end::process_effect_combat_end(state)
        }
        EffectKind::TurnStart => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_turn_start::process_effect_turn_start(id_target, state)
        }
        EffectKind::TurnEnd => {
            if id_target == Some(state.id_character) {
                debug_assert!(matches!(state.active, Screen::Combat));
                process_effect_turn_end::process_effect_turn_end_character(state)
            } else {
                process_effect_turn_end::process_effect_turn_end_monster(id_target, state)
            }
        }
        EffectKind::MoveUpdate => {
            process_effect_move_update::process_effect_move_update(id_target, state)
        }
        EffectKind::MoveExecute => {
            process_effect_move_execute::process_effect_move_execute(id_target, state)
        }
        EffectKind::RoomEnter => process_effect_room_enter::process_effect_room_enter(state),
        EffectKind::RestSiteExit => {
            process_effect_rest_site_exit::process_effect_rest_site_exit(state)
        }
        EffectKind::MonsterSpawn { name } => {
            process_effect_monster_spawn::process_effect_monster_spawn(id_source, state, name)
        }
        EffectKind::EscapeMonster => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_escape_monster::process_effect_escape_monster(id_target, state)
        }
        EffectKind::GoldSteal { amount } => {
            process_effect_gold_steal::process_effect_gold_steal(id_source, state, amount)
        }
        EffectKind::HexaghostBurnIncrease { count } => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_hexaghost_burn_increase::process_effect_hexaghost_burn_increase(
                state, count,
            )
        }
        EffectKind::HexaghostDivider => {
            process_effect_hexaghost_divider::process_effect_hexaghost_divider(id_source, state)
        }
        EffectKind::GoldGain { amount } => {
            process_effect_gold_gain::process_effect_gold_gain(state, amount)
        }
        EffectKind::RoomSelect => {
            process_effect_room_select::process_effect_room_select(id_target, state)
        }
        EffectKind::CardRemoveFromDeck => {
            process_effect_card_remove_from_deck::process_effect_card_remove_from_deck(
                id_target, state,
            )
        }
        EffectKind::CardAddToDeck {
            card_name,
            upgraded,
        } => process_effect_card_add_to_deck::process_effect_card_add_to_deck(
            state, card_name, upgraded,
        ),
        EffectKind::MaxHealthGain { amount } => {
            process_effect_max_health_gain::process_effect_max_health_gain(id_target, state, amount)
        }
        EffectKind::MaxHealthLoss { amount } => {
            process_effect_max_health_loss::process_effect_max_health_loss(id_target, state, amount)
        }
        EffectKind::ChestOpen => process_effect_chest_open::process_effect_chest_open(state),
        EffectKind::PotionUse => {
            process_effect_potion_use::process_effect_potion_use(id_target, state)
        }
        EffectKind::PotionAddRandom { limited } => {
            process_effect_potion_add_random::process_effect_potion_add_random(state, limited)
        }
        EffectKind::CardDiscoverSelect { kind, count } => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_card_discover_select::process_effect_card_discover_select(
                state,
                kind,
                CardColor::Green, // TODO: other characters
                count,
            );
        }
        EffectKind::GoldLoss { amount } => {
            process_effect_gold_loss::process_effect_gold_loss(state, amount)
        }
        EffectKind::HealthGainPct { numer, denom } => {
            process_effect_health_gain_pct::process_effect_health_gain_pct(state, numer, denom)
        }
        EffectKind::HealthLossPct { numer, denom } => {
            process_effect_health_loss_pct::process_effect_health_loss_pct(state, numer, denom)
        }
        EffectKind::MaxHealthLossPct { numer, denom } => {
            process_effect_max_health_loss_pct::process_effect_max_health_loss_pct(
                state, numer, denom,
            )
        }
        EffectKind::CardUpgradeRandomInDeck { count } => {
            process_effect_card_upgrade_random_in_deck::process_effect_card_upgrade_random_in_deck(
                state, count,
            )
        }
        EffectKind::CardTransformRoll => {
            process_effect_card_transform_roll::process_effect_card_transform_roll(state)
        }
        EffectKind::RelicGrantRandom { tier } => {
            process_effect_relic_grant_random::process_effect_relic_grant_random(state, tier)
        }
        EffectKind::RelicGrantSpecific {
            name,
            fallback_circlet,
        } => process_effect_relic_grant_specific::process_effect_relic_grant_specific(
            state,
            name,
            fallback_circlet,
        ),
        EffectKind::EventAdvanceState { delta } => {
            process_effect_event_advance_state::process_effect_event_advance_state(
                id_source, state, delta,
            )
        }
        EffectKind::RollD100Branch {
            chance,
            on_lt,
            on_ge,
        } => process_effect_roll_d100_branch::process_effect_roll_d100_branch(
            id_source, state, chance, on_lt, on_ge,
        ),
        EffectKind::EventEnd => {
            process_effect_event_end::process_effect_event_end(id_source, state)
        }
        EffectKind::DeckSelectStart { kind } => {
            process_effect_deck_select_start::process_effect_deck_select_start(state, kind)
        }
        EffectKind::CardDiscoverPick => {
            debug_assert!(matches!(state.active, Screen::Combat));
            process_effect_card_discover_pick::process_effect_card_discover_pick(id_target, state)
        }
        EffectKind::DeckSelectPick { kind: ds_kind } => {
            process_effect_deck_select_pick::process_effect_deck_select_pick(
                id_target, state, ds_kind,
            )
        }
        EffectKind::Noop => panic!("Noop effect should never be dispatched"),
    }
}

pub fn process_queue(state: &mut GameState) {
    while !state.game_over {
        let Some(effect) = state.effect_queue.pop_front() else {
            return; // Queue drained
        };
        if process_effect(state, effect) {
            return; // Queue halted
        }
    }
}
