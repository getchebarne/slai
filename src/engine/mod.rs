pub mod process_card_play;
pub mod process_effect_accuracy_resync;
pub mod process_effect_act_transition;
pub mod process_effect_adventurer_search;
pub mod process_effect_block_gain;
pub mod process_effect_block_set;
pub mod process_effect_bomb_arm;
pub mod process_effect_bomb_tick;
pub mod process_effect_bonfire_offer;
pub mod process_effect_card_add;
pub mod process_effect_card_add_random;
pub mod process_effect_card_adopt;
pub mod process_effect_card_bottle;
pub mod process_effect_card_cost_minus_discards;
pub mod process_effect_card_discard;
pub mod process_effect_card_discover_pick;
pub mod process_effect_card_discover_roll;
pub mod process_effect_card_draw;
pub mod process_effect_card_draw_if_no_attacks;
pub mod process_effect_card_draw_up_to;
pub mod process_effect_card_duplicate;
pub mod process_effect_card_exhaust;
pub mod process_effect_card_free_play_spend;
pub mod process_effect_card_move;
pub mod process_effect_card_nightmare_pick;
pub mod process_effect_card_nightmare_spawn;
pub mod process_effect_card_place;
pub mod process_effect_card_play_from_card_pile_draw_top;
pub mod process_effect_card_play_relocate;
pub mod process_effect_card_purge;
pub mod process_effect_card_remove;
pub mod process_effect_card_retain;
pub mod process_effect_card_setup_pick;
pub mod process_effect_card_stasis_return;
pub mod process_effect_card_stasis_steal;
pub mod process_effect_card_transform;
pub mod process_effect_card_upgrade;
pub mod process_effect_chest_open;
pub mod process_effect_combat_end;
pub mod process_effect_combat_start;
pub mod process_effect_damage_deal;
pub mod process_effect_damage_finisher;
pub mod process_effect_damage_flechettes;
pub mod process_effect_damage_physical;
pub mod process_effect_death;
pub mod process_effect_debuffs_clear;
pub mod process_effect_defensive_mode;
pub mod process_effect_distraction_add;
pub mod process_effect_du_vu_doll_recount;
pub mod process_effect_energy_delta;
pub mod process_effect_escape_plan_check;
pub mod process_effect_event_advance_state;
pub mod process_effect_event_consume;
pub mod process_effect_gamble;
pub mod process_effect_gamble_draw;
pub mod process_effect_girya_lift;
pub mod process_effect_glass_knife_decay;
pub mod process_effect_gold_delta;
pub mod process_effect_gold_steal;
pub mod process_effect_gremlin_summon;
pub mod process_effect_hand_of_greed_proc;
pub mod process_effect_health_delta;
pub mod process_effect_health_lower_to;
pub mod process_effect_heel_hook_proc;
pub mod process_effect_hexaghost_burn_increase;
pub mod process_effect_joust_bet;
pub mod process_effect_knowing_skull_cost_bump;
pub mod process_effect_lagavulin_wake;
pub mod process_effect_lifesteal_heal;
pub mod process_effect_match_flip_seen;
pub mod process_effect_match_flip_unseen;
pub mod process_effect_mausoleum_open;
pub mod process_effect_max_health_delta;
pub mod process_effect_mayhem_proc;
pub mod process_effect_modifier_delta;
pub mod process_effect_modifier_multiply;
pub mod process_effect_modifier_remove;
pub mod process_effect_modifier_tick;
pub mod process_effect_monster_escape;
pub mod process_effect_monster_remove;
pub mod process_effect_monster_spawn;
pub mod process_effect_monster_split;
pub mod process_effect_move_execute;
pub mod process_effect_move_update;
pub mod process_effect_poison_tick;
pub mod process_effect_potion_add_random;
pub mod process_effect_potion_adopt;
pub mod process_effect_potion_discard;
pub mod process_effect_potion_use;
pub mod process_effect_relic_adopt;
pub mod process_effect_relic_grant_pool;
pub mod process_effect_relic_grant_random;
pub mod process_effect_relic_grant_specific;
pub mod process_effect_relic_lose;
pub mod process_effect_relic_reward_remove_one;
pub mod process_effect_rest_dig;
pub mod process_effect_rest_site_consume;
pub mod process_effect_rest_smith;
pub mod process_effect_rest_toke;
pub mod process_effect_reward_roll_cards;
pub mod process_effect_reward_roll_gold;
pub mod process_effect_reward_roll_potions;
pub mod process_effect_reward_roll_relic;
pub mod process_effect_reward_take;
pub mod process_effect_ritual_dagger_proc;
pub mod process_effect_room_enter;
pub mod process_effect_room_exit;
pub mod process_effect_room_select;
pub mod process_effect_scrap_ooze_reach;
pub mod process_effect_set_cost_override;
pub mod process_effect_shop_build;
pub mod process_effect_shop_buy;
pub mod process_effect_shop_purge;
pub mod process_effect_shuffle_card_pile_discard_into_card_pile_draw;
pub mod process_effect_singing_bowl_proc;
pub mod process_effect_sneaky_strike_proc;
pub mod process_effect_storm_of_steel_proc;
pub mod process_effect_strength_lose_temp;
pub mod process_effect_turn_end_character;
pub mod process_effect_turn_end_monster;
pub mod process_effect_turn_monsters;
pub mod process_effect_turn_start_character;
pub mod process_effect_turn_start_monster;
pub mod process_effect_unceasing_top_draw;
pub mod process_effect_unload_discard;
pub mod process_effect_wheel_spin;

use self::process_card_play::process_card_play;
use self::process_effect_accuracy_resync::process_effect_accuracy_resync;
use self::process_effect_act_transition::process_effect_act_transition;
use self::process_effect_adventurer_search::process_effect_adventurer_search;
use self::process_effect_block_gain::process_effect_block_gain;
use self::process_effect_block_set::process_effect_block_set;
use self::process_effect_bomb_arm::process_effect_bomb_arm;
use self::process_effect_bomb_tick::process_effect_bomb_tick;
use self::process_effect_bonfire_offer::process_effect_bonfire_offer;
use self::process_effect_card_add::process_effect_card_add;
use self::process_effect_card_add_random::process_effect_card_add_random;
use self::process_effect_card_adopt::process_effect_card_adopt;
use self::process_effect_card_bottle::process_effect_card_bottle;
use self::process_effect_card_cost_minus_discards::process_effect_card_cost_minus_discards;
use self::process_effect_card_discard::process_effect_card_discard;
use self::process_effect_card_discover_pick::process_effect_card_discover_pick;
use self::process_effect_card_discover_roll::process_effect_card_discover_roll;
use self::process_effect_card_draw::process_effect_card_draw;
use self::process_effect_card_draw_if_no_attacks::process_effect_card_draw_if_no_attacks;
use self::process_effect_card_draw_up_to::process_effect_card_draw_up_to;
use self::process_effect_card_duplicate::process_effect_card_duplicate;
use self::process_effect_card_exhaust::process_effect_card_exhaust;
use self::process_effect_card_free_play_spend::process_effect_card_free_play_spend;
use self::process_effect_card_move::process_effect_card_move;
use self::process_effect_card_nightmare_pick::process_effect_card_nightmare_pick;
use self::process_effect_card_nightmare_spawn::process_effect_card_nightmare_spawn;
use self::process_effect_card_place::process_effect_card_place;
use self::process_effect_card_play_from_card_pile_draw_top::process_effect_card_play_from_card_pile_draw_top;
use self::process_effect_card_play_relocate::process_effect_card_play_relocate;
use self::process_effect_card_purge::process_effect_card_purge;
use self::process_effect_card_remove::process_effect_card_remove;
use self::process_effect_card_retain::process_effect_card_retain;
use self::process_effect_card_setup_pick::process_effect_card_setup_pick;
use self::process_effect_card_stasis_return::process_effect_card_stasis_return;
use self::process_effect_card_stasis_steal::process_effect_card_stasis_steal;
use self::process_effect_card_transform::process_effect_card_transform;
use self::process_effect_card_upgrade::process_effect_card_upgrade;
use self::process_effect_chest_open::process_effect_chest_open;
use self::process_effect_combat_end::process_effect_combat_end;
use self::process_effect_combat_start::process_effect_combat_start;
use self::process_effect_damage_deal::process_effect_damage_deal;
use self::process_effect_damage_finisher::process_effect_damage_finisher;
use self::process_effect_damage_flechettes::process_effect_damage_flechettes;
use self::process_effect_damage_physical::process_effect_damage_physical;
use self::process_effect_death::process_effect_death;
use self::process_effect_debuffs_clear::process_effect_debuffs_clear;
use self::process_effect_defensive_mode::process_effect_defensive_mode;
use self::process_effect_distraction_add::process_effect_distraction_add;
use self::process_effect_du_vu_doll_recount::process_effect_du_vu_doll_recount;
use self::process_effect_energy_delta::process_effect_energy_delta;
use self::process_effect_escape_plan_check::process_effect_escape_plan_check;
use self::process_effect_event_advance_state::process_effect_event_advance_state;
use self::process_effect_event_consume::process_effect_event_consume;
use self::process_effect_gamble::process_effect_gamble;
use self::process_effect_gamble_draw::process_effect_gamble_draw;
use self::process_effect_girya_lift::process_effect_girya_lift;
use self::process_effect_glass_knife_decay::process_effect_glass_knife_decay;
use self::process_effect_gold_delta::process_effect_gold_delta;
use self::process_effect_gold_steal::process_effect_gold_steal;
use self::process_effect_gremlin_summon::process_effect_gremlin_summon;
use self::process_effect_hand_of_greed_proc::process_effect_hand_of_greed_proc;
use self::process_effect_health_delta::process_effect_health_delta;
use self::process_effect_health_lower_to::process_effect_health_lower_to;
use self::process_effect_heel_hook_proc::process_effect_heel_hook_proc;
use self::process_effect_hexaghost_burn_increase::process_effect_hexaghost_burn_increase;
use self::process_effect_joust_bet::process_effect_joust_bet;
use self::process_effect_knowing_skull_cost_bump::process_effect_knowing_skull_cost_bump;
use self::process_effect_lagavulin_wake::process_effect_lagavulin_wake;
use self::process_effect_lifesteal_heal::process_effect_lifesteal_heal;
use self::process_effect_match_flip_seen::process_effect_match_flip_seen;
use self::process_effect_match_flip_unseen::process_effect_match_flip_unseen;
use self::process_effect_mausoleum_open::process_effect_mausoleum_open;
use self::process_effect_max_health_delta::process_effect_max_health_delta;
use self::process_effect_mayhem_proc::process_effect_mayhem_proc;
use self::process_effect_modifier_delta::process_effect_modifier_delta;
use self::process_effect_modifier_multiply::process_effect_modifier_multiply;
use self::process_effect_modifier_remove::process_effect_modifier_remove;
use self::process_effect_modifier_tick::process_effect_modifier_tick;
use self::process_effect_monster_escape::process_effect_monster_escape;
use self::process_effect_monster_remove::process_effect_monster_remove;
use self::process_effect_monster_spawn::process_effect_monster_spawn;
use self::process_effect_monster_split::process_effect_monster_split;
use self::process_effect_move_execute::process_effect_move_execute;
use self::process_effect_move_update::process_effect_move_update;
use self::process_effect_poison_tick::process_effect_poison_tick;
use self::process_effect_potion_add_random::process_effect_potion_add_random;
use self::process_effect_potion_adopt::process_effect_potion_adopt;
use self::process_effect_potion_discard::process_effect_potion_discard;
use self::process_effect_potion_use::process_effect_potion_use;
use self::process_effect_relic_adopt::process_effect_relic_adopt;
use self::process_effect_relic_grant_pool::process_effect_relic_grant_pool;
use self::process_effect_relic_grant_random::process_effect_relic_grant_random;
use self::process_effect_relic_grant_specific::process_effect_relic_grant_specific;
use self::process_effect_relic_lose::process_effect_relic_lose;
use self::process_effect_relic_reward_remove_one::process_effect_relic_reward_remove_one;
use self::process_effect_rest_dig::process_effect_rest_dig;
use self::process_effect_rest_site_consume::process_effect_rest_site_consume;
use self::process_effect_rest_smith::process_effect_rest_smith;
use self::process_effect_rest_toke::process_effect_rest_toke;
use self::process_effect_reward_roll_cards::process_effect_reward_roll_cards;
use self::process_effect_reward_roll_gold::process_effect_reward_roll_gold;
use self::process_effect_reward_roll_potions::process_effect_reward_roll_potions;
use self::process_effect_reward_roll_relic::process_effect_reward_roll_relic;
use self::process_effect_reward_take::process_effect_reward_take;
use self::process_effect_ritual_dagger_proc::process_effect_ritual_dagger_proc;
use self::process_effect_room_enter::process_effect_room_enter;
use self::process_effect_room_exit::process_effect_room_exit;
use self::process_effect_room_select::process_effect_room_select;
use self::process_effect_scrap_ooze_reach::process_effect_scrap_ooze_reach;
use self::process_effect_set_cost_override::process_effect_set_cost_override;
use self::process_effect_shop_build::process_effect_shop_build;
use self::process_effect_shop_buy::process_effect_shop_buy;
use self::process_effect_shop_purge::process_effect_shop_purge;
use self::process_effect_shuffle_card_pile_discard_into_card_pile_draw::process_effect_shuffle_card_pile_discard_into_card_pile_draw;
use self::process_effect_singing_bowl_proc::process_effect_singing_bowl_proc;
use self::process_effect_sneaky_strike_proc::process_effect_sneaky_strike_proc;
use self::process_effect_storm_of_steel_proc::process_effect_storm_of_steel_proc;
use self::process_effect_strength_lose_temp::process_effect_strength_lose_temp;
use self::process_effect_turn_end_character::process_effect_turn_end_character;
use self::process_effect_turn_end_monster::process_effect_turn_end_monster;
use self::process_effect_turn_monsters::process_effect_turn_monsters;
use self::process_effect_turn_start_character::process_effect_turn_start_character;
use self::process_effect_turn_start_monster::process_effect_turn_start_monster;
use self::process_effect_unceasing_top_draw::process_effect_unceasing_top_draw;
use self::process_effect_unload_discard::process_effect_unload_discard;
use self::process_effect_wheel_spin::process_effect_wheel_spin;

// Shared shop-stock machinery (not a processor)
mod shop;

use std::collections::VecDeque;

use rand::Rng;

use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::effect::is_multi_pick;
use crate::effect::pool_is_cards;
use crate::game::GameState;
use crate::game::Location;
use crate::map::get_active_room_kind;
use crate::types::Combat;
use crate::types::Event;
use crate::types::EventName;
use crate::types::RoomKind;
use crate::utils::filter_candidates;
use crate::utils::shuffle;
use crate::utils::unceasing_top_fires;

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
    combat: &Combat,
    event: &Event,
    id_card_pile_deck: &[usize],
) {
    // Combat-scoped pools demand the combat context; Character/Source/CardPileDeck don't
    match candidate_pool {
        CandidatePool::CardPileHand => {
            assert!(combat.active, "CardPileHand pool outside combat");
            effect_candidate_buf.extend_from_slice(&combat.id_card_pile_hand)
        }
        CandidatePool::CardPileDraw => {
            assert!(combat.active, "CardPileDraw pool outside combat");
            effect_candidate_buf.extend_from_slice(&combat.id_card_pile_draw)
        }
        CandidatePool::CardPileDiscard => {
            assert!(combat.active, "CardPileDiscard pool outside combat");
            effect_candidate_buf.extend_from_slice(&combat.id_card_pile_discard)
        }
        CandidatePool::CardPileExhaust => {
            assert!(combat.active, "CardPileExhaust pool outside combat");
            effect_candidate_buf.extend_from_slice(&combat.id_card_pile_exhaust)
        }
        CandidatePool::Character => effect_candidate_buf.push(id_character),
        CandidatePool::Monsters => {
            assert!(combat.active, "Monsters pool outside combat");
            effect_candidate_buf.extend(combat.id_monsters.iter().flatten().copied())
        }
        CandidatePool::MonsterPicked => {
            assert!(combat.active, "MonsterPicked pool outside combat");
            effect_candidate_buf.extend(combat.id_monster_picked)
        }
        CandidatePool::Source => {
            let id_source = id_source
                .expect("Attempted to resolve `CandidatePool::Source` without `id_source`");

            effect_candidate_buf.push(id_source)
        }
        CandidatePool::CardPileDiscover => {
            assert!(combat.active, "CardPileDiscover pool outside combat");
            effect_candidate_buf.extend_from_slice(&combat.id_card_pile_discover)
        }
        CandidatePool::CardPileDeck => effect_candidate_buf.extend_from_slice(id_card_pile_deck),
        CandidatePool::CardPileEventRoll => {
            assert!(event.active, "CardPileEventRoll pool outside an event");
            effect_candidate_buf.extend_from_slice(&event.id_card_pile_event_roll)
        }
        CandidatePool::RelicEventRoll => {
            assert!(event.active, "RelicEventRoll pool outside an event");
            effect_candidate_buf.extend_from_slice(&event.id_relic_event_roll)
        }
        CandidatePool::PotionEventRoll => {
            assert!(event.active, "PotionEventRoll pool outside an event");
            effect_candidate_buf.extend_from_slice(&event.id_potion_event_roll)
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
        SelectionKind::InputUpTo { count } => {
            assert!(count > 0, "InputUpTo requires a positive count");
            effect_candidate_buf.is_empty()
        }
    }
}

// Returns true on success; false on halt (unresolved Effect stashed in effect_pending)
pub fn process_effect(state: &mut GameState, effect: Effect) -> bool {
    let id_target = match effect.target {
        Target::Direct(id_target) => id_target,
        Target::Resolve {
            candidate_pool,
            filters,
            selection_kind,
        } => {
            let resolved = resolve_or_halt(
                state,
                effect.id_source,
                candidate_pool,
                filters,
                selection_kind,
            );
            if resolved {
                // Targets are in `effect_candidate_buf`
                enqueue_direct_targets(
                    effect.id_source,
                    &state.effect_candidate_buf,
                    effect.kind,
                    &mut state.effect_queue,
                );
            } else {
                // A hand pick drops the waiting plays of Cards still in the hand
                if candidate_pool == CandidatePool::CardPileHand {
                    let id_card_pile_hand = &state.combat.id_card_pile_hand;
                    state
                        .card_play_queue
                        .retain(|card_play| !id_card_pile_hand.contains(&card_play.id_card));
                }

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
    filters: &[CandidateFilter],
    selection_kind: SelectionKind,
) -> bool {
    assert!(
        !is_multi_pick(selection_kind) || pool_is_cards(candidate_pool),
        "multi-pick halt over a non-card pool: {candidate_pool:?}"
    );

    // Stage 1: the pool enumerates
    state.effect_candidate_buf.clear();
    fill_buf_candidates(
        &mut state.effect_candidate_buf,
        candidate_pool,
        id_source,
        state.id_character,
        &state.combat,
        &state.event,
        &state.id_card_pile_deck,
    );

    // Stage 2: the filters retain
    filter_candidates(
        filters,
        &mut state.effect_candidate_buf,
        &state.entities,
        id_source,
    );

    // Nothing survived: the effect resolves to no targets (guards Single's assert)
    if state.effect_candidate_buf.is_empty() {
        return true;
    }

    // Stage 3: the selection picks. Returns `true` if the targets were resolved
    let resolved = resolve_selection_kind(
        &mut state.effect_candidate_buf,
        selection_kind,
        &mut state.rng,
    );

    // A Hand pick that takes every Card takes them from the right
    if resolved
        && candidate_pool == CandidatePool::CardPileHand
        && matches!(selection_kind, SelectionKind::Input { .. })
    {
        state.effect_candidate_buf.reverse();
    }
    resolved
}

fn dispatch_by_kind(
    state: &mut GameState,
    kind: EffectKind,
    id_source: Option<usize>,
    id_target: Option<usize>,
) {
    match kind {
        EffectKind::CardDraw { count } => process_effect_card_draw(state, count),
        EffectKind::CardDrawIfNoAttacks { count } => {
            process_effect_card_draw_if_no_attacks(state, count)
        }
        EffectKind::CardAddRandom {
            color,
            kind,
            card_pile,
            count,
            cost_zero,
            upgraded,
            rarity,
        } => process_effect_card_add_random(
            state, color, kind, card_pile, count, cost_zero, upgraded, rarity,
        ),
        EffectKind::HandOfGreedProc { gold } => {
            process_effect_hand_of_greed_proc(id_target, state, gold)
        }
        EffectKind::CardDrawUpTo { amount } => process_effect_card_draw_up_to(state, amount),
        EffectKind::CardAdd {
            card_name,
            card_pile,
            count,
            upgraded,
        } => process_effect_card_add(state, card_name, card_pile, count, upgraded),
        EffectKind::CardDiscard { source } => process_effect_card_discard(id_target, state, source),
        EffectKind::CardMove {
            card_pile,
            cost_zero,
        } => process_effect_card_move(id_target, state, card_pile, cost_zero),
        EffectKind::DamageMindBlast => {
            unreachable!("Mind Blast's play turns its hit into DamagePhysical")
        }
        EffectKind::ShuffleCardPileDiscardIntoCardPileDraw => {
            process_effect_shuffle_card_pile_discard_into_card_pile_draw(state)
        }
        EffectKind::CardRetain => process_effect_card_retain(id_target, state),
        EffectKind::CardSetupPick { free, bottom } => {
            process_effect_card_setup_pick(id_target, state, free, bottom)
        }
        EffectKind::CardNightmarePick => process_effect_card_nightmare_pick(id_target, state),
        EffectKind::CardNightmareSpawn => process_effect_card_nightmare_spawn(id_target, state),
        EffectKind::CardPlace { pile } => process_effect_card_place(id_target, state, pile),
        EffectKind::CardExhaust => process_effect_card_exhaust(id_target, state),
        EffectKind::CardPlayFromCardPileDrawTop => {
            process_effect_card_play_from_card_pile_draw_top(id_target, state)
        }
        EffectKind::BombArm { turns, damage } => process_effect_bomb_arm(state, turns, damage),
        EffectKind::BombTick { seq } => process_effect_bomb_tick(state, seq),
        EffectKind::LifestealHeal => process_effect_lifesteal_heal(id_target, state),
        EffectKind::CardPlayRelocate => process_effect_card_play_relocate(id_target, state),
        EffectKind::CardRemove => process_effect_card_remove(id_target, state),
        EffectKind::ActTransition => process_effect_act_transition(state),
        EffectKind::AdventurerSearch => process_effect_adventurer_search(state),
        EffectKind::BonfireOffer => process_effect_bonfire_offer(id_target, state),
        EffectKind::CardBottle => process_effect_card_bottle(id_target, state),
        EffectKind::CardCostMinusDiscards => {
            process_effect_card_cost_minus_discards(id_target, state)
        }
        EffectKind::CardFreePlaySpend => process_effect_card_free_play_spend(id_target, state),
        EffectKind::GiryaLift => process_effect_girya_lift(state),
        EffectKind::SingingBowlProc { idx_bundle } => {
            process_effect_singing_bowl_proc(state, idx_bundle)
        }
        EffectKind::WheelSpin => process_effect_wheel_spin(state),
        EffectKind::CardUpgrade => process_effect_card_upgrade(id_target, state),
        EffectKind::AccuracyResync => process_effect_accuracy_resync(id_target, state),
        EffectKind::RewardRollCards { bundles, trigger } => {
            process_effect_reward_roll_cards(state, bundles, trigger)
        }
        EffectKind::RewardRollGold { amount } => process_effect_reward_roll_gold(state, amount),
        EffectKind::RewardRollPotions { count, trigger } => {
            process_effect_reward_roll_potions(state, count, trigger)
        }
        EffectKind::RelicRewardRemoveOne => process_effect_relic_reward_remove_one(state),
        EffectKind::RewardRollRelic { pick, exclusion } => {
            process_effect_reward_roll_relic(state, pick, exclusion)
        }
        EffectKind::RitualDaggerProc { bump } => {
            process_effect_ritual_dagger_proc(id_source, id_target, state, bump)
        }
        EffectKind::RewardTake { kind } => process_effect_reward_take(id_target, state, kind),
        EffectKind::RoomExit => process_effect_room_exit(state),
        EffectKind::RestDig => process_effect_rest_dig(state),
        EffectKind::RestSiteConsume => process_effect_rest_site_consume(state),
        EffectKind::RestSmith => process_effect_rest_smith(id_target, state),
        EffectKind::RestToke => process_effect_rest_toke(id_target, state),
        EffectKind::DamagePhysical {
            amount,
            instances,
            lifesteal,
        } => process_effect_damage_physical(
            id_source, id_target, state, amount, instances, false, lifesteal,
        ),
        EffectKind::DamagePhysicalIfPoisoned { amount } => {
            process_effect_damage_physical(id_source, id_target, state, amount, 1, true, false)
        }
        EffectKind::GlassKnifeDecay { delta } => {
            process_effect_glass_knife_decay(id_target, state, delta)
        }
        EffectKind::DistractionAdd => process_effect_distraction_add(state),
        EffectKind::DuVuDollRecount => process_effect_du_vu_doll_recount(state),
        EffectKind::SetCostOverride {
            amount,
            only_reduce,
            random,
            scope,
        } => process_effect_set_cost_override(id_target, state, amount, only_reduce, random, scope),
        EffectKind::EscapePlanCheck { block } => {
            process_effect_escape_plan_check(id_source, state, block)
        }
        EffectKind::DamageFinisher { damage } => {
            process_effect_damage_finisher(id_source, id_target, state, damage)
        }
        EffectKind::DamageFlechettes { damage } => {
            process_effect_damage_flechettes(id_source, id_target, state, damage)
        }
        EffectKind::HeelHookProc => process_effect_heel_hook_proc(id_target, state),
        EffectKind::SneakyStrikeProc { energy } => process_effect_sneaky_strike_proc(state, energy),
        EffectKind::StormOfSteelProc { upgraded } => {
            process_effect_storm_of_steel_proc(state, upgraded)
        }
        EffectKind::StrengthLoseTemp { stacks } => {
            process_effect_strength_lose_temp(id_target, state, stacks)
        }
        EffectKind::UnloadDiscard => process_effect_unload_discard(state),
        EffectKind::DamageDeal { amount, lifesteal } => {
            process_effect_damage_deal(id_source, id_target, state, amount, lifesteal)
        }
        EffectKind::HealthDelta { sign, amount } => {
            process_effect_health_delta(id_source, id_target, state, sign, amount)
        }
        EffectKind::HealthLowerTo { amount } => {
            process_effect_health_lower_to(id_target, state, amount)
        }
        EffectKind::BlockGain { amount } => {
            process_effect_block_gain(id_source, id_target, state, amount)
        }
        EffectKind::BlockSet { amount } => process_effect_block_set(id_target, state, amount),
        EffectKind::EnergyDelta { sign, amount } => {
            process_effect_energy_delta(state, sign, amount)
        }
        EffectKind::ModifierDelta { kind, stacks } => {
            process_effect_modifier_delta(id_source, id_target, state, kind, stacks)
        }
        EffectKind::ModifierMultiply { kind, factor } => {
            process_effect_modifier_multiply(id_target, state, kind, factor)
        }
        EffectKind::ModifierRemove { kind } => {
            process_effect_modifier_remove(id_target, state, kind)
        }
        EffectKind::ModifierTick => process_effect_modifier_tick(id_target, state),
        EffectKind::PoisonTick { amount } => process_effect_poison_tick(id_target, state, amount),
        EffectKind::Death { with_leader } => {
            // Character can die outside Combat; empty Monster slots make iter a no-op
            process_effect_death(id_target, state, with_leader)
        }
        EffectKind::DefensiveMode => process_effect_defensive_mode(id_target, state),
        EffectKind::LagavulinWake => process_effect_lagavulin_wake(id_target, state),
        EffectKind::CombatStart { elite } => process_effect_combat_start(state, elite),
        EffectKind::CombatEnd { escaped_character } => {
            process_effect_combat_end(state, escaped_character)
        }
        EffectKind::TurnStartCharacter => process_effect_turn_start_character(state),
        EffectKind::TurnStartMonster => process_effect_turn_start_monster(id_target, state),
        EffectKind::TurnEndCharacter { landing } => {
            process_effect_turn_end_character(state, landing)
        }
        EffectKind::TurnEndMonster => process_effect_turn_end_monster(id_target, state),
        EffectKind::TurnMonsters { stage } => process_effect_turn_monsters(state, stage),
        EffectKind::UnceasingTopDraw => process_effect_unceasing_top_draw(state),
        EffectKind::MoveUpdate { move_override } => {
            process_effect_move_update(id_target, state, move_override)
        }
        EffectKind::MoveExecute => process_effect_move_execute(id_target, state),
        EffectKind::RoomEnter { location, landing } => {
            process_effect_room_enter(state, location, landing)
        }
        EffectKind::MonsterSpawn { name, minion, slot } => {
            process_effect_monster_spawn(state, name, minion, slot)
        }
        EffectKind::MonsterSplit { name, slot_offset } => {
            process_effect_monster_split(id_source, state, name, slot_offset)
        }
        EffectKind::MonsterEscape => process_effect_monster_escape(id_target, state),
        EffectKind::MonsterRemove => process_effect_monster_remove(id_target, state),
        EffectKind::GoldSteal { amount } => process_effect_gold_steal(id_source, state, amount),
        EffectKind::GremlinSummon => process_effect_gremlin_summon(state),
        EffectKind::DebuffsClear => process_effect_debuffs_clear(id_target, state),
        EffectKind::CardStasisReturn { hand_full } => {
            process_effect_card_stasis_return(id_target, state, hand_full)
        }
        EffectKind::CardStasisSteal => process_effect_card_stasis_steal(id_source, state),
        EffectKind::JoustBet { on_owner } => process_effect_joust_bet(state, on_owner),
        EffectKind::KnowingSkullCostBump => {
            process_effect_knowing_skull_cost_bump(id_source, state)
        }
        EffectKind::MausoleumOpen => process_effect_mausoleum_open(state),
        EffectKind::MatchFlipSeen => process_effect_match_flip_seen(id_target, state),
        EffectKind::MatchFlipUnseen => process_effect_match_flip_unseen(state),
        EffectKind::MayhemProc => process_effect_mayhem_proc(id_target, state),
        EffectKind::HexaghostBurnIncrease { count } => {
            process_effect_hexaghost_burn_increase(state, count)
        }
        EffectKind::GoldDelta { sign, amount } => process_effect_gold_delta(state, sign, amount),
        EffectKind::RoomSelect => process_effect_room_select(id_target, state),
        EffectKind::CardPurge => process_effect_card_purge(id_target, state),
        EffectKind::CardDuplicate { card_pile } => {
            process_effect_card_duplicate(id_target, state, card_pile)
        }
        EffectKind::CardTransform { upgraded } => {
            process_effect_card_transform(id_target, state, upgraded)
        }
        EffectKind::CardAdopt { landing } => process_effect_card_adopt(id_target, state, landing),
        EffectKind::MaxHealthDelta { sign, amount } => {
            process_effect_max_health_delta(id_target, state, sign, amount)
        }
        EffectKind::ChestOpen => process_effect_chest_open(state),
        EffectKind::PotionDiscard => process_effect_potion_discard(id_target, state),
        EffectKind::ShopBuild => process_effect_shop_build(state),
        EffectKind::ShopBuy { slot } => process_effect_shop_buy(id_target, state, slot),
        EffectKind::ShopPurge => process_effect_shop_purge(state),
        EffectKind::PotionUse { at_pick } => {
            process_effect_potion_use(id_source, id_target, state, at_pick)
        }
        EffectKind::PotionAddRandom { limited, uniform } => {
            process_effect_potion_add_random(state, limited, uniform)
        }
        EffectKind::PotionAdopt => process_effect_potion_adopt(id_target, state),
        EffectKind::CardDiscoverRoll {
            kind,
            color,
            exclude,
            count,
        } => {
            process_effect_card_discover_roll(state, kind, color, exclude, count);
        }
        EffectKind::Gamble { choose_discards } => process_effect_gamble(state, choose_discards),
        EffectKind::GambleDraw { discards_before } => {
            process_effect_gamble_draw(state, discards_before)
        }
        EffectKind::RelicGrantPool { pool } => process_effect_relic_grant_pool(state, pool),
        EffectKind::RelicGrantRandom { tier, exclusion } => {
            process_effect_relic_grant_random(state, tier, exclusion)
        }
        EffectKind::RelicGrantSpecific { name } => process_effect_relic_grant_specific(state, name),
        EffectKind::RelicLose => process_effect_relic_lose(id_target, state),
        EffectKind::RelicAdopt => process_effect_relic_adopt(id_target, state),
        EffectKind::EventAdvanceState { delta } => process_effect_event_advance_state(state, delta),
        EffectKind::ScrapOozeReach {
            chance,
            advance_on_miss,
        } => process_effect_scrap_ooze_reach(state, chance, advance_on_miss),
        EffectKind::EventConsume => process_effect_event_consume(state),
        EffectKind::CardDiscoverPick {
            cost_zero,
            card_pile,
            copies,
        } => process_effect_card_discover_pick(id_target, state, cost_zero, card_pile, copies),
        EffectKind::NoOp => panic!("NoOp effect should never be dispatched"),
    }
}

pub fn process_effect_queue(state: &mut GameState) {
    while !state.game_over {
        let Some(effect) = state.effect_queue.pop_front() else {
            // The next waiting Card play starts once every queued effect has resolved
            if let Some(card_play) = state.card_play_queue.pop_front() {
                process_card_play(state, card_play);
                continue;
            }

            // The next turn phase starts once every queued effect and waiting Card play has resolved
            if let Some(phase) = state.phase_queue.pop_front() {
                state.effect_queue.push_back(phase);
                continue;
            }

            // Unceasing Top: an empty hand at queue rest draws 1 and keeps going
            if unceasing_top_fires(state) {
                state.effect_queue.push_back(Effect {
                    kind: EffectKind::CardDraw { count: 1 },
                    id_source: None,
                    target: Target::Direct(None),
                });
                continue;
            }
            ensure_context_validity(state);
            return; // Every queue drained
        };
        if !process_effect(state, effect) {
            ensure_context_validity(state);
            return; // Queue halted
        }
    }
}

// Cross-source witness: the active contexts must agree with each other and
// with world facts. Every active context is checked against the Room directly
fn ensure_context_validity(state: &GameState) {
    assert!(
        state.effect_pending.is_some() || state.effect_pending_selected.is_empty(),
        "staged picks outlived their halt"
    );
    if state.game_over {
        return;
    }
    let room_kind = get_active_room_kind(&state.id_rooms, state.location, &state.entities);

    // At most one Room context owns the visit
    let room_contexts_active = [
        state.shop.active,
        state.chest.active,
        state.rest_site.active,
        state.event.active,
    ]
    .iter()
    .filter(|&&a| a)
    .count();
    assert!(
        room_contexts_active <= 1,
        "Two room contexts active at once"
    );

    // Combat and Reward never coexist, and combat never runs inside a shop,
    // chest, or rest site — only over an event (its fight) or the bare Room
    assert!(
        !(state.combat.active && state.reward.active),
        "Combat and Reward both active"
    );
    assert!(
        !(state.combat.active
            && (state.shop.active || state.chest.active || state.rest_site.active)),
        "Combat active inside a non-event room context"
    );

    // Card plays and turn phases wait only inside a combat
    assert!(
        state.combat.active || state.card_play_queue.is_empty(),
        "Card plays queued outside combat"
    );
    assert!(
        state.combat.active || state.phase_queue.is_empty(),
        "Turn phases queued outside combat"
    );

    // A Reward overlays a consumed event; a fight stacks over an unconsumed one
    if state.reward.active && state.event.active {
        assert!(
            state.event.consumed,
            "Reward staged over an unconsumed event"
        );
    }
    if state.combat.active && state.event.active {
        assert!(
            !state.event.consumed,
            "Combat running over a consumed event"
        );
    }

    // "?" Rooms keep RoomKind::Unknown on the map after resolving
    if state.rest_site.active {
        assert!(
            room_kind == Some(RoomKind::RestSite),
            "RestSite context inconsistent with room kind {:?} at {:?}",
            room_kind,
            state.location
        );
    }
    if state.chest.active {
        assert!(
            matches!(room_kind, Some(RoomKind::Treasure | RoomKind::Unknown)),
            "Chest context inconsistent with room kind {:?} at {:?}",
            room_kind,
            state.location
        );
    }
    if state.shop.active {
        assert!(
            matches!(room_kind, Some(RoomKind::Shop | RoomKind::Unknown)),
            "Shop context inconsistent with room kind {:?} at {:?}",
            room_kind,
            state.location
        );
    }
    if state.event.active {
        // Neow rests over Location::Start, before any Room exists
        let ok = if matches!(state.event.name, EventName::Neow) {
            state.location == Location::Start
        } else {
            matches!(room_kind, Some(RoomKind::EventRoom | RoomKind::Unknown))
        };
        assert!(
            ok,
            "Event context inconsistent with room kind {:?} at {:?}",
            room_kind, state.location
        );
    }
    if state.combat.active {
        assert!(
            matches!(
                room_kind,
                Some(
                    RoomKind::CombatMonster
                        | RoomKind::CombatElite
                        | RoomKind::CombatBoss
                        | RoomKind::EventRoom
                        | RoomKind::Unknown
                )
            ),
            "Combat context inconsistent with room kind {:?} at {:?}",
            room_kind,
            state.location
        );
    }
    // Treasure/RestSite/Shop: chest loot, Dream Catcher's rest reward, and
    // Orrery over the stock; Location::Start: Neow's staged offers
    if state.reward.active {
        let ok = matches!(
            room_kind,
            Some(
                RoomKind::CombatMonster
                    | RoomKind::CombatElite
                    | RoomKind::CombatBoss
                    | RoomKind::EventRoom
                    | RoomKind::Treasure
                    | RoomKind::RestSite
                    | RoomKind::Shop
                    | RoomKind::Unknown
            )
        ) || state.location == Location::Start;
        assert!(
            ok,
            "Reward context inconsistent with room kind {:?} at {:?}",
            room_kind, state.location
        );
    }
}
