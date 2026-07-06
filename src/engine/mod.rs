pub mod process_effect_block_gain;
pub mod process_effect_block_set;
pub mod process_effect_calculated_gamble;
pub mod process_effect_card_add_to_deck;
pub mod process_effect_card_add_to_discard;
pub mod process_effect_card_add_to_hand;
pub mod process_effect_card_adopt;
pub mod process_effect_card_discard;
pub mod process_effect_card_discover_pick;
pub mod process_effect_card_discover_roll;
pub mod process_effect_card_draw;
pub mod process_effect_card_draw_up_to;
pub mod process_effect_card_duplicate;
pub mod process_effect_card_exhaust;
pub mod process_effect_card_move_to_discard;
pub mod process_effect_card_nightmare_pick;
pub mod process_effect_card_nightmare_spawn;
pub mod process_effect_card_play;
pub mod process_effect_card_purge;
pub mod process_effect_card_remove;
pub mod process_effect_card_retain;
pub mod process_effect_card_setup_pick;
pub mod process_effect_card_transform;
pub mod process_effect_card_upgrade;
pub mod process_effect_chest_open;
pub mod process_effect_combat_end;
pub mod process_effect_combat_start;
pub mod process_effect_damage_deal;
pub mod process_effect_damage_finisher;
pub mod process_effect_damage_flechettes;
pub mod process_effect_damage_mind_blast;
pub mod process_effect_damage_physical;
pub mod process_effect_death;
pub mod process_effect_distraction_add;
pub mod process_effect_energy_gain;
pub mod process_effect_energy_loss;
pub mod process_effect_escape_plan_check;
pub mod process_effect_event_advance_state;
pub mod process_effect_event_consume;
pub mod process_effect_glass_knife_decay;
pub mod process_effect_gold_delta;
pub mod process_effect_gold_steal;
pub mod process_effect_health_delta;
pub mod process_effect_heel_hook_proc;
pub mod process_effect_hexaghost_burn_increase;
pub mod process_effect_hexaghost_divider;
pub mod process_effect_max_health_delta;
pub mod process_effect_modifier_gain;
pub mod process_effect_modifier_multiply;
pub mod process_effect_modifier_remove;
pub mod process_effect_modifier_set_not_new;
pub mod process_effect_modifier_tick;
pub mod process_effect_monster_escape;
pub mod process_effect_monster_spawn;
pub mod process_effect_move_execute;
pub mod process_effect_move_update;
pub mod process_effect_poison_tick;
pub mod process_effect_potion_add_random;
pub mod process_effect_potion_adopt;
pub mod process_effect_potion_discard;
pub mod process_effect_potion_use;
pub mod process_effect_relic_adopt;
pub mod process_effect_relic_grant_random;
pub mod process_effect_relic_grant_specific;
pub mod process_effect_rest_site_consume;
pub mod process_effect_reward_roll_chest;
pub mod process_effect_reward_roll_combat;
pub mod process_effect_reward_take;
pub mod process_effect_room_enter;
pub mod process_effect_room_exit;
pub mod process_effect_room_select;
pub mod process_effect_scrap_ooze_reach;
pub mod process_effect_set_cost_override;
pub mod process_effect_shop_build;
pub mod process_effect_shop_buy_card;
pub mod process_effect_shop_buy_potion;
pub mod process_effect_shop_buy_relic;
pub mod process_effect_shop_purge;
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

use crate::consts::MAX_MONSTERS;
use crate::effect::CandidatePool;
use crate::effect::CandidatePoolMonstersFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::types::CardColor;
use crate::types::RelicName;
use crate::types::Screen;
use crate::utils::deck_filter_matches;
use crate::utils::shuffle;

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

fn fill_buf_candidates(
    effect_candidate_buf: &mut Vec<usize>,
    candidate_pool: CandidatePool,
    id_source: Option<usize>,
    id_character: usize,
    id_hand: &[usize],
    id_picked_monster: Option<usize>,
    id_monsters: &[Option<usize>; MAX_MONSTERS],
    entities: &[Entity],
    id_discover: &[usize],
    id_deck: &[usize],
) {
    match candidate_pool {
        CandidatePool::Hand => effect_candidate_buf.extend_from_slice(id_hand),
        CandidatePool::Character => effect_candidate_buf.push(id_character),
        CandidatePool::Monsters { filter } => match filter {
            CandidatePoolMonstersFilter::All => {
                effect_candidate_buf.extend(id_monsters.iter().flatten().copied())
            }
            CandidatePoolMonstersFilter::Other => {
                let id_source =
                    id_source.expect("CandidatePool::Monsters{Other} requires id_source");
                // Iterate all monsters skipping the one that sourced the effect
                for id_monster in id_monsters.iter().flatten().copied() {
                    if id_monster != id_source {
                        effect_candidate_buf.push(id_monster);
                    }
                }
                // Last monster standing falls back to targeting itself
                if effect_candidate_buf.is_empty() {
                    effect_candidate_buf.push(id_source);
                }
            }
            CandidatePoolMonstersFilter::Picked => effect_candidate_buf.push(
                id_picked_monster
                    .expect("CandidatePool::Monsters{Picked} requires id_picked_monster"),
            ),
        },
        CandidatePool::Source => {
            let id_source = id_source
                .expect("Attempted to resolve `CandidatePool::Source` without `id_source`");

            effect_candidate_buf.push(id_source)
        }
        CandidatePool::Discover => effect_candidate_buf.extend_from_slice(id_discover),
        CandidatePool::Deck { filter } => {
            for &id in id_deck {
                if deck_filter_matches(filter, &entities[id]) {
                    effect_candidate_buf.push(id);
                }
            }
        }
    }
}

// Returns true if resolved (ready to enqueue); false if halted on player input
fn resolve_selection_kind(
    effect_candidate_buf: &mut Vec<usize>,
    selection_kind: SelectionKind,
    rng: &mut impl Rng,
) -> bool {
    match selection_kind {
        SelectionKind::All => true,
        SelectionKind::Single => {
            assert_eq!(
                effect_candidate_buf.len(),
                1,
                "SelectionKind::Single resolved to {} candidates",
                effect_candidate_buf.len()
            );
            true
        }
        SelectionKind::Random { count } => {
            shuffle(effect_candidate_buf.as_mut_slice(), rng);
            effect_candidate_buf.truncate(count as usize);
            true
        }
        SelectionKind::Input { count } => (count as usize) >= effect_candidate_buf.len(),
    }
}

// Returns true on success; false on halt (unresolved Effect stashed in effect_pending)
pub fn process_effect(state: &mut GameState, effect: Effect) -> bool {
    let id_target = match effect.target {
        Target::Direct(id_target) => id_target,
        Target::Resolve {
            candidate_pool,
            selection_kind,
        } => {
            let resolved = resolve_or_halt(state, effect.id_source, candidate_pool, selection_kind);
            if resolved {
                // Targets are in `effect_candidate_buf`
                enqueue_direct_targets(
                    effect.id_source,
                    &state.effect_candidate_buf,
                    effect.kind,
                    &mut state.effect_queue,
                );
            } else {
                // Effect needs player input to be resolved
                state.effect_pending = Some(effect);
            }
            return resolved;
        }
    };

    dispatch_by_kind(state, effect.kind, effect.id_source, id_target);
    true
}

fn resolve_or_halt(
    state: &mut GameState,
    id_source: Option<usize>,
    candidate_pool: CandidatePool,
    selection_kind: SelectionKind,
) -> bool {
    // Clear candidate buffer and re-fill it
    state.effect_candidate_buf.clear();
    fill_buf_candidates(
        &mut state.effect_candidate_buf,
        candidate_pool,
        id_source,
        state.id_character,
        &state.id_hand,
        state.id_picked_monster,
        &state.id_monsters,
        &state.entities,
        &state.id_discover,
        &state.id_deck,
    );

    // Resolve `SelectionKind`. Returns `true` if the effect's targets were resolved
    resolve_selection_kind(
        &mut state.effect_candidate_buf,
        selection_kind,
        &mut state.rng,
    )
}

fn dispatch_by_kind(
    state: &mut GameState,
    kind: EffectKind,
    id_source: Option<usize>,
    id_target: Option<usize>,
) {
    match kind {
        EffectKind::CardDraw { count } => {
            process_effect_card_draw::process_effect_card_draw(state, count)
        }
        EffectKind::CardDrawUpTo { amount } => {
            process_effect_card_draw_up_to::process_effect_card_draw_up_to(state, amount)
        }
        EffectKind::CardPlay => {
            process_effect_card_play::process_effect_card_play(id_target, state)
        }
        EffectKind::CardAddToDiscard {
            card_name,
            count,
            upgraded,
        } => {
            process_effect_card_add_to_discard::process_effect_card_add_to_discard(
                state, card_name, count, upgraded,
            )
        }
        EffectKind::CardDiscard { source } => {
            process_effect_card_discard::process_effect_card_discard(id_target, state, source)
        }
        EffectKind::CardMoveToDiscard => {
            process_effect_card_move_to_discard::process_effect_card_move_to_discard(
                id_target, state,
            )
        }
        EffectKind::DamageMindBlast => {
            process_effect_damage_mind_blast::process_effect_damage_mind_blast(
                id_source, id_target, state,
            )
        }
        EffectKind::ShuffleDiscardPileIntoDrawPile => {
            process_effect_shuffle_discard_pile_into_draw_pile::process_effect_shuffle_discard_pile_into_draw_pile(state)
        }
        EffectKind::CardRetain => {
            process_effect_card_retain::process_effect_card_retain(id_target, state)
        }
        EffectKind::CardSetupPick => {
            process_effect_card_setup_pick::process_effect_card_setup_pick(id_target, state)
        }
        EffectKind::CardNightmarePick => {
            process_effect_card_nightmare_pick::process_effect_card_nightmare_pick(id_target, state)
        }
        EffectKind::CardNightmareSpawn => {
            process_effect_card_nightmare_spawn::process_effect_card_nightmare_spawn(state)
        }
        EffectKind::CardExhaust => {
            process_effect_card_exhaust::process_effect_card_exhaust(id_target, state)
        }
        EffectKind::CardRemove => {
            process_effect_card_remove::process_effect_card_remove(id_target, state)
        }
        EffectKind::CardAddToHand {
            card_name,
            count,
            upgraded,
        } => {
            process_effect_card_add_to_hand::process_effect_card_add_to_hand(
                state, card_name, count, upgraded,
            )
        }
        EffectKind::CalculatedGamble => {
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
            process_effect_reward_take::process_effect_reward_take(id_target, state, kind)
        }
        EffectKind::RoomExit => process_effect_room_exit::process_effect_room_exit(state),
        EffectKind::RestSiteConsume => {
            process_effect_rest_site_consume::process_effect_rest_site_consume(id_target, state)
        }
        EffectKind::TargetSet => {
            process_effect_target_set::process_effect_target_set(id_target, state)
        }
        EffectKind::TargetClear => {
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
            process_effect_distraction_add::process_effect_distraction_add(state)
        }
        EffectKind::SetCostOverride { amount } => {
            process_effect_set_cost_override::process_effect_set_cost_override(
                id_target, state, amount,
            )
        }
        EffectKind::EscapePlanCheck { block } => {
            process_effect_escape_plan_check::process_effect_escape_plan_check(state, block)
        }
        EffectKind::DamageFinisher { damage } => {
            process_effect_damage_finisher::process_effect_damage_finisher(
                id_source, id_target, state, damage,
            )
        }
        EffectKind::DamageFlechettes { damage } => {
            process_effect_damage_flechettes::process_effect_damage_flechettes(
                id_source, id_target, state, damage,
            )
        }
        EffectKind::HeelHookProc => {
            process_effect_heel_hook_proc::process_effect_heel_hook_proc(id_target, state)
        }
        EffectKind::SneakyStrikeProc { energy } => {
            process_effect_sneaky_strike_proc::process_effect_sneaky_strike_proc(state, energy)
        }
        EffectKind::StormOfSteelProc { upgraded } => {
            process_effect_storm_of_steel_proc::process_effect_storm_of_steel_proc(state, upgraded)
        }
        EffectKind::UnloadDiscard => {
            process_effect_unload_discard::process_effect_unload_discard(state)
        }
        EffectKind::DamageDeal { amount } => {
            process_effect_damage_deal::process_effect_damage_deal(
                id_source, id_target, state, amount,
            )
        }
        EffectKind::HealthDelta { sign, amount } => {
            process_effect_health_delta::process_effect_health_delta(id_target, state, sign, amount)
        }
        EffectKind::BlockGain { amount } => process_effect_block_gain::process_effect_block_gain(
            id_source, id_target, state, amount,
        ),
        EffectKind::BlockSet { amount } => {
            process_effect_block_set::process_effect_block_set(id_target, state, amount)
        }
        EffectKind::EnergyGain { amount } => {
            process_effect_energy_gain::process_effect_energy_gain(state, amount)
        }
        EffectKind::EnergyLoss { amount } => {
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
            // Character can die outside Combat; empty monster slots make iter a no-op
            process_effect_death::process_effect_death(id_target, state)
        }
        EffectKind::CombatStart => {
            process_effect_combat_start::process_effect_combat_start(state)
        }
        EffectKind::CombatEnd => {
            process_effect_combat_end::process_effect_combat_end(state)
        }
        EffectKind::TurnStart => {
            process_effect_turn_start::process_effect_turn_start(id_target, state)
        }
        EffectKind::TurnEnd => {
            if id_target == Some(state.id_character) {
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
        EffectKind::MonsterSpawn { name } => {
            process_effect_monster_spawn::process_effect_monster_spawn(id_source, state, name)
        }
        EffectKind::MonsterEscape => {
            process_effect_monster_escape::process_effect_monster_escape(id_target, state)
        }
        EffectKind::GoldSteal { amount } => {
            process_effect_gold_steal::process_effect_gold_steal(id_source, state, amount)
        }
        EffectKind::HexaghostBurnIncrease { count } => {
            process_effect_hexaghost_burn_increase::process_effect_hexaghost_burn_increase(
                state, count,
            )
        }
        EffectKind::HexaghostDivider => {
            process_effect_hexaghost_divider::process_effect_hexaghost_divider(id_source, state)
        }
        EffectKind::GoldDelta { sign, amount } => {
            process_effect_gold_delta::process_effect_gold_delta(state, sign, amount)
        }
        EffectKind::RoomSelect => {
            process_effect_room_select::process_effect_room_select(id_target, state)
        }
        EffectKind::CardPurge => {
            process_effect_card_purge::process_effect_card_purge(id_target, state)
        }
        EffectKind::CardDuplicate => {
            process_effect_card_duplicate::process_effect_card_duplicate(id_target, state)
        }
        EffectKind::CardTransform => {
            process_effect_card_transform::process_effect_card_transform(id_target, state)
        }
        EffectKind::CardAddToDeck {
            card_name,
            upgraded,
        } => process_effect_card_add_to_deck::process_effect_card_add_to_deck(
            state, card_name, upgraded,
        ),
        EffectKind::CardAdopt => {
            process_effect_card_adopt::process_effect_card_adopt(id_target, state)
        }
        EffectKind::MaxHealthDelta { sign, amount } => {
            process_effect_max_health_delta::process_effect_max_health_delta(
                id_target, state, sign, amount,
            )
        }
        EffectKind::ChestOpen => process_effect_chest_open::process_effect_chest_open(state),
        EffectKind::PotionDiscard => {
            process_effect_potion_discard::process_effect_potion_discard(id_target, state)
        }
        EffectKind::ShopBuild => {
            process_effect_shop_build::process_effect_shop_build(state)
        }
        EffectKind::ShopBuyCard => {
            process_effect_shop_buy_card::process_effect_shop_buy_card(id_target, state)
        }
        EffectKind::ShopBuyPotion => {
            process_effect_shop_buy_potion::process_effect_shop_buy_potion(id_target, state)
        }
        EffectKind::ShopBuyRelic => {
            process_effect_shop_buy_relic::process_effect_shop_buy_relic(id_target, state)
        }
        EffectKind::ShopPurge => {
            process_effect_shop_purge::process_effect_shop_purge(id_target, state)
        }
        EffectKind::PotionUse => {
            process_effect_potion_use::process_effect_potion_use(id_target, state)
        }
        EffectKind::PotionAddRandom { limited } => {
            process_effect_potion_add_random::process_effect_potion_add_random(state, limited)
        }
        EffectKind::PotionAdopt => {
            process_effect_potion_adopt::process_effect_potion_adopt(id_target, state)
        }
        EffectKind::CardDiscoverRoll { kind, count } => {
            process_effect_card_discover_roll::process_effect_card_discover_roll(
                state,
                kind,
                CardColor::Green, // TODO: other characters
                count,
            );
        }
        EffectKind::RelicGrantRandom => {
            process_effect_relic_grant_random::process_effect_relic_grant_random(state)
        }
        EffectKind::RelicGrantSpecific {
            name,
            fallback_circlet,
        } => process_effect_relic_grant_specific::process_effect_relic_grant_specific(
            state,
            name,
            fallback_circlet,
        ),
        EffectKind::RelicAdopt => {
            process_effect_relic_adopt::process_effect_relic_adopt(id_target, state)
        }
        EffectKind::EventAdvanceState { delta } => {
            process_effect_event_advance_state::process_effect_event_advance_state(
                id_source, state, delta,
            )
        }
        EffectKind::ScrapOozeReach {
            dmg,
            chance,
            advance_on_miss,
        } => process_effect_scrap_ooze_reach::process_effect_scrap_ooze_reach(
            id_source,
            state,
            dmg,
            chance,
            advance_on_miss,
        ),
        EffectKind::EventConsume => {
            process_effect_event_consume::process_effect_event_consume(id_source, state)
        }
        EffectKind::CardDiscoverPick => {
            process_effect_card_discover_pick::process_effect_card_discover_pick(id_target, state)
        }
        EffectKind::NoOp => panic!("NoOp effect should never be dispatched"),
    }
}

pub fn process_effect_queue(state: &mut GameState) {
    while !state.game_over {
        let Some(effect) = state.effect_queue.pop_front() else {
            // Unceasing Top: an empty hand at queue rest draws 1 and keeps going
            if unceasing_top_fires(state) {
                state.effect_queue.push_back(Effect {
                    kind: EffectKind::CardDraw { count: 1 },
                    id_source: None,
                    target: Target::Direct(None),
                });
                continue;
            }
            return; // Queue drained
        };
        if !process_effect(state, effect) {
            return; // Queue halted
        }
    }
}

// Queue rest in Combat means the player is about to act; a drawable card ends the loop
fn unceasing_top_fires(state: &GameState) -> bool {
    state.id_relics[RelicName::UnceasingTop as usize].is_some()
        && matches!(state.screen, Screen::Combat)
        && state.effect_pending.is_none()
        && state.id_hand.is_empty()
        && !(state.id_pile_draw.is_empty() && state.id_pile_discard.is_empty())
        && !modifier_has(
            &state.entities[state.id_character].modifiers,
            ModifierKind::NoDraw,
        )
}
