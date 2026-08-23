pub mod process_effect_act_transition;
pub mod process_effect_adventurer_search;
pub mod process_effect_block_gain;
pub mod process_effect_block_set;
pub mod process_effect_bonfire_offer;
pub mod process_effect_card_add;
pub mod process_effect_card_add_random;
pub mod process_effect_card_adopt;
pub mod process_effect_card_bottle;
pub mod process_effect_card_discard;
pub mod process_effect_card_discover_pick;
pub mod process_effect_card_discover_roll;
pub mod process_effect_card_draw;
pub mod process_effect_card_draw_if_no_attacks;
pub mod process_effect_card_draw_up_to;
pub mod process_effect_card_duplicate;
pub mod process_effect_card_exhaust;
pub mod process_effect_card_move;
pub mod process_effect_card_nightmare_pick;
pub mod process_effect_card_nightmare_spawn;
pub mod process_effect_card_play;
pub mod process_effect_card_play_from_draw_top;
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
pub mod process_effect_debuffs_clear;
pub mod process_effect_distraction_add;
pub mod process_effect_energy_delta;
pub mod process_effect_escape_plan_check;
pub mod process_effect_event_advance_state;
pub mod process_effect_event_consume;
pub mod process_effect_gamble;
pub mod process_effect_girya_lift;
pub mod process_effect_glass_knife_decay;
pub mod process_effect_gold_delta;
pub mod process_effect_gold_steal;
pub mod process_effect_gremlin_summon;
pub mod process_effect_hand_of_greed_proc;
pub mod process_effect_health_delta;
pub mod process_effect_health_set;
pub mod process_effect_heel_hook_proc;
pub mod process_effect_hexaghost_burn_increase;
pub mod process_effect_joust_bet;
pub mod process_effect_knowing_skull_cost_bump;
pub mod process_effect_mausoleum_open;
pub mod process_effect_max_health_delta;
pub mod process_effect_modifier_gain;
pub mod process_effect_modifier_multiply;
pub mod process_effect_modifier_remove;
pub mod process_effect_modifier_set_not_new;
pub mod process_effect_modifier_tick;
pub mod process_effect_monster_escape;
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
pub mod process_effect_rest_site_consume;
pub mod process_effect_reward_roll_cards;
pub mod process_effect_reward_roll_gold;
pub mod process_effect_reward_roll_library_cards;
pub mod process_effect_reward_roll_neow_cards;
pub mod process_effect_reward_roll_potion;
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
pub mod process_effect_shuffle_discard_pile_into_draw_pile;
pub mod process_effect_singing_bowl_proc;
pub mod process_effect_sneaky_strike_proc;
pub mod process_effect_stasis_steal;
pub mod process_effect_storm_of_steel_proc;
pub mod process_effect_strength_lose_temp;
pub mod process_effect_target_clear;
pub mod process_effect_target_set;
pub mod process_effect_turn_end;
pub mod process_effect_turn_start;
pub mod process_effect_unload_discard;
pub mod process_effect_wheel_spin;

use self::process_effect_act_transition::process_effect_act_transition;
use self::process_effect_adventurer_search::process_effect_adventurer_search;
use self::process_effect_block_gain::process_effect_block_gain;
use self::process_effect_block_set::process_effect_block_set;
use self::process_effect_bonfire_offer::process_effect_bonfire_offer;
use self::process_effect_card_add::process_effect_card_add;
use self::process_effect_card_add_random::process_effect_card_add_random;
use self::process_effect_card_adopt::process_effect_card_adopt;
use self::process_effect_card_bottle::process_effect_card_bottle;
use self::process_effect_card_discard::process_effect_card_discard;
use self::process_effect_card_discover_pick::process_effect_card_discover_pick;
use self::process_effect_card_discover_roll::process_effect_card_discover_roll;
use self::process_effect_card_draw::process_effect_card_draw;
use self::process_effect_card_draw_if_no_attacks::process_effect_card_draw_if_no_attacks;
use self::process_effect_card_draw_up_to::process_effect_card_draw_up_to;
use self::process_effect_card_duplicate::process_effect_card_duplicate;
use self::process_effect_card_exhaust::process_effect_card_exhaust;
use self::process_effect_card_move::process_effect_card_move;
use self::process_effect_card_nightmare_pick::process_effect_card_nightmare_pick;
use self::process_effect_card_nightmare_spawn::process_effect_card_nightmare_spawn;
use self::process_effect_card_play::process_effect_card_play;
use self::process_effect_card_play_from_draw_top::process_effect_card_play_from_draw_top;
use self::process_effect_card_purge::process_effect_card_purge;
use self::process_effect_card_remove::process_effect_card_remove;
use self::process_effect_card_retain::process_effect_card_retain;
use self::process_effect_card_setup_pick::process_effect_card_setup_pick;
use self::process_effect_card_transform::process_effect_card_transform;
use self::process_effect_card_upgrade::process_effect_card_upgrade;
use self::process_effect_chest_open::process_effect_chest_open;
use self::process_effect_combat_end::process_effect_combat_end;
use self::process_effect_combat_start::process_effect_combat_start;
use self::process_effect_damage_deal::process_effect_damage_deal;
use self::process_effect_damage_finisher::process_effect_damage_finisher;
use self::process_effect_damage_flechettes::process_effect_damage_flechettes;
use self::process_effect_damage_mind_blast::process_effect_damage_mind_blast;
use self::process_effect_damage_physical::process_effect_damage_physical;
use self::process_effect_death::process_effect_death;
use self::process_effect_debuffs_clear::process_effect_debuffs_clear;
use self::process_effect_distraction_add::process_effect_distraction_add;
use self::process_effect_energy_delta::process_effect_energy_delta;
use self::process_effect_escape_plan_check::process_effect_escape_plan_check;
use self::process_effect_event_advance_state::process_effect_event_advance_state;
use self::process_effect_event_consume::process_effect_event_consume;
use self::process_effect_gamble::process_effect_gamble;
use self::process_effect_girya_lift::process_effect_girya_lift;
use self::process_effect_glass_knife_decay::process_effect_glass_knife_decay;
use self::process_effect_gold_delta::process_effect_gold_delta;
use self::process_effect_gold_steal::process_effect_gold_steal;
use self::process_effect_gremlin_summon::process_effect_gremlin_summon;
use self::process_effect_hand_of_greed_proc::process_effect_hand_of_greed_proc;
use self::process_effect_health_delta::process_effect_health_delta;
use self::process_effect_health_set::process_effect_health_set;
use self::process_effect_heel_hook_proc::process_effect_heel_hook_proc;
use self::process_effect_hexaghost_burn_increase::process_effect_hexaghost_burn_increase;
use self::process_effect_joust_bet::process_effect_joust_bet;
use self::process_effect_knowing_skull_cost_bump::process_effect_knowing_skull_cost_bump;
use self::process_effect_mausoleum_open::process_effect_mausoleum_open;
use self::process_effect_max_health_delta::process_effect_max_health_delta;
use self::process_effect_modifier_gain::process_effect_modifier_gain;
use self::process_effect_modifier_multiply::process_effect_modifier_multiply;
use self::process_effect_modifier_remove::process_effect_modifier_remove;
use self::process_effect_modifier_set_not_new::process_effect_modifier_set_not_new;
use self::process_effect_modifier_tick::process_effect_modifier_tick;
use self::process_effect_monster_escape::process_effect_monster_escape;
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
use self::process_effect_rest_site_consume::process_effect_rest_site_consume;
use self::process_effect_reward_roll_cards::process_effect_reward_roll_cards;
use self::process_effect_reward_roll_gold::process_effect_reward_roll_gold;
use self::process_effect_reward_roll_library_cards::process_effect_reward_roll_library_cards;
use self::process_effect_reward_roll_neow_cards::process_effect_reward_roll_neow_cards;
use self::process_effect_reward_roll_potion::process_effect_reward_roll_potion;
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
use self::process_effect_shuffle_discard_pile_into_draw_pile::process_effect_shuffle_discard_pile_into_draw_pile;
use self::process_effect_singing_bowl_proc::process_effect_singing_bowl_proc;
use self::process_effect_sneaky_strike_proc::process_effect_sneaky_strike_proc;
use self::process_effect_stasis_steal::process_effect_stasis_steal;
use self::process_effect_storm_of_steel_proc::process_effect_storm_of_steel_proc;
use self::process_effect_strength_lose_temp::process_effect_strength_lose_temp;
use self::process_effect_target_clear::process_effect_target_clear;
use self::process_effect_target_set::process_effect_target_set;
use self::process_effect_turn_end::process_effect_turn_end;
use self::process_effect_turn_start::process_effect_turn_start;
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
use crate::game::GameState;
use crate::game::Location;
use crate::map::get_active_room_kind;
use crate::types::Combat;
use crate::types::Event;
use crate::types::EventName;
use crate::types::RoomKind;
use crate::utils::candidate_matches;
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
    id_card_deck: &[usize],
) {
    // Combat-scoped pools demand the combat context; Character/Source/Deck don't
    match candidate_pool {
        CandidatePool::Hand => {
            assert!(combat.active, "Hand pool outside combat");
            effect_candidate_buf.extend_from_slice(&combat.id_card_hand)
        }
        CandidatePool::PileDraw => {
            assert!(combat.active, "PileDraw pool outside combat");
            effect_candidate_buf.extend_from_slice(&combat.id_card_draw)
        }
        CandidatePool::PileDiscard => {
            assert!(combat.active, "PileDiscard pool outside combat");
            effect_candidate_buf.extend_from_slice(&combat.id_card_discard)
        }
        CandidatePool::PileExhaust => {
            assert!(combat.active, "PileExhaust pool outside combat");
            effect_candidate_buf.extend_from_slice(&combat.id_card_exhaust)
        }
        CandidatePool::Character => effect_candidate_buf.push(id_character),
        CandidatePool::Monsters => {
            assert!(combat.active, "Monsters pool outside combat");
            effect_candidate_buf.extend(combat.id_monsters.iter().flatten().copied())
        }
        CandidatePool::Source => {
            let id_source = id_source
                .expect("Attempted to resolve `CandidatePool::Source` without `id_source`");

            effect_candidate_buf.push(id_source)
        }
        CandidatePool::Discover => {
            assert!(combat.active, "Discover pool outside combat");
            effect_candidate_buf.extend_from_slice(&combat.id_card_discover)
        }
        CandidatePool::Deck => effect_candidate_buf.extend_from_slice(id_card_deck),
        CandidatePool::EventRollCard => {
            assert!(event.active, "EventRollCard pool outside an event");
            effect_candidate_buf.extend_from_slice(&event.id_roll_card)
        }
        CandidatePool::EventRollRelic => {
            assert!(event.active, "EventRollRelic pool outside an event");
            effect_candidate_buf.extend_from_slice(&event.id_roll_relic)
        }
        CandidatePool::EventRollPotion => {
            assert!(event.active, "EventRollPotion pool outside an event");
            effect_candidate_buf.extend_from_slice(&event.id_roll_potion)
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
            if count == 0 {
                effect_candidate_buf.clear();
            }
            count == 0 || effect_candidate_buf.is_empty()
        }
    }
}

// Returns true on success; false on halt (unresolved Effect stashed in effect_pending)
pub fn process_effect(state: &mut GameState, effect: Effect) -> bool {
    let id_target = match effect.target {
        Target::Direct(id_target) => id_target,
        Target::Resolve {
            candidate_pool,
            filter,
            selection_kind,
        } => {
            let resolved = resolve_or_halt(
                state,
                effect.id_source,
                candidate_pool,
                filter,
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
    filter: CandidateFilter,
    selection_kind: SelectionKind,
) -> bool {
    // Stage 1: the pool enumerates
    state.effect_candidate_buf.clear();
    fill_buf_candidates(
        &mut state.effect_candidate_buf,
        candidate_pool,
        id_source,
        state.id_character,
        &state.combat,
        &state.event,
        &state.id_card_deck,
    );

    // Stage 2: the filter retains
    let id_monster_picked = if state.combat.active {
        state.combat.id_monster_picked
    } else {
        None
    };
    let entities = &state.entities;
    state
        .effect_candidate_buf
        .retain(|&id| candidate_matches(filter, id, &entities[id], id_source, id_monster_picked));

    // NotSource: the last monster standing falls back to targeting itself
    if filter == CandidateFilter::NotSource
        && state.effect_candidate_buf.is_empty()
        && let Some(id_source) = id_source
    {
        state.effect_candidate_buf.push(id_source);
    }

    // Nothing survived: the effect resolves to no targets (guards Single's assert)
    if state.effect_candidate_buf.is_empty() {
        return true;
    }

    // Stage 3: the selection picks. Returns `true` if the targets were resolved
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
        EffectKind::CardDraw { count } => process_effect_card_draw(state, count),
        EffectKind::CardDrawIfNoAttacks { count } => {
            process_effect_card_draw_if_no_attacks(state, count)
        }
        EffectKind::CardAddRandom {
            color,
            kind,
            pile,
            count,
            cost_zero,
            upgraded,
            rarity,
        } => process_effect_card_add_random(
            state, color, kind, pile, count, cost_zero, upgraded, rarity,
        ),
        EffectKind::HandOfGreedProc { gold } => {
            process_effect_hand_of_greed_proc(id_target, state, gold)
        }
        EffectKind::CardDrawUpTo { amount } => process_effect_card_draw_up_to(state, amount),
        EffectKind::CardPlay => process_effect_card_play(id_target, state),
        EffectKind::CardAdd {
            card_name,
            pile,
            count,
            upgraded,
        } => process_effect_card_add(state, card_name, pile, count, upgraded),
        EffectKind::CardDiscard { source } => process_effect_card_discard(id_target, state, source),
        EffectKind::CardMove { pile, cost_zero } => {
            process_effect_card_move(id_target, state, pile, cost_zero)
        }
        EffectKind::DamageMindBlast => {
            process_effect_damage_mind_blast(id_source, id_target, state)
        }
        EffectKind::ShuffleDiscardPileIntoDrawPile => {
            process_effect_shuffle_discard_pile_into_draw_pile(state)
        }
        EffectKind::CardRetain => process_effect_card_retain(id_target, state),
        EffectKind::CardSetupPick { free, bottom } => {
            process_effect_card_setup_pick(id_target, state, free, bottom)
        }
        EffectKind::CardNightmarePick => process_effect_card_nightmare_pick(id_target, state),
        EffectKind::CardNightmareSpawn => process_effect_card_nightmare_spawn(state),
        EffectKind::CardExhaust => process_effect_card_exhaust(id_target, state),
        EffectKind::CardPlayFromDrawTop => process_effect_card_play_from_draw_top(state),
        EffectKind::CardRemove => process_effect_card_remove(id_target, state),
        EffectKind::ActTransition => process_effect_act_transition(state),
        EffectKind::AdventurerSearch => process_effect_adventurer_search(state),
        EffectKind::BonfireOffer => process_effect_bonfire_offer(id_target, state),
        EffectKind::CardBottle => process_effect_card_bottle(id_target, state),
        EffectKind::GiryaLift => process_effect_girya_lift(state),
        EffectKind::SingingBowlProc { idx_bundle } => {
            process_effect_singing_bowl_proc(state, idx_bundle)
        }
        EffectKind::WheelSpin => process_effect_wheel_spin(state),
        EffectKind::CardUpgrade => process_effect_card_upgrade(id_target, state),
        EffectKind::RewardRollCards { bundles, rare_only } => {
            process_effect_reward_roll_cards(state, bundles, rare_only)
        }
        EffectKind::RewardRollGold { amount } => process_effect_reward_roll_gold(state, amount),
        EffectKind::RewardRollLibraryCards => process_effect_reward_roll_library_cards(state),
        EffectKind::RewardRollNeowCards {
            colorless,
            rare_only,
        } => process_effect_reward_roll_neow_cards(state, colorless, rare_only),
        EffectKind::RewardRollPotion { eligible } => {
            process_effect_reward_roll_potion(state, eligible)
        }
        EffectKind::RewardRollPotions { count, uniform } => {
            process_effect_reward_roll_potions(state, count, uniform)
        }
        EffectKind::RewardRollRelic { pick } => process_effect_reward_roll_relic(state, pick),
        EffectKind::RitualDaggerProc { bump } => {
            process_effect_ritual_dagger_proc(id_source, id_target, state, bump)
        }
        EffectKind::RewardTake { kind } => process_effect_reward_take(id_target, state, kind),
        EffectKind::RoomExit => process_effect_room_exit(state),
        EffectKind::RestSiteConsume => process_effect_rest_site_consume(state),
        EffectKind::TargetSet => process_effect_target_set(id_target, state),
        EffectKind::TargetClear => process_effect_target_clear(state),
        EffectKind::DamagePhysical { amount, lifesteal } => {
            process_effect_damage_physical(id_source, id_target, state, amount, false, lifesteal)
        }
        EffectKind::DamagePhysicalIfPoisoned { amount } => {
            process_effect_damage_physical(id_source, id_target, state, amount, true, false)
        }
        EffectKind::GlassKnifeDecay { delta } => {
            process_effect_glass_knife_decay(id_target, state, delta)
        }
        EffectKind::DistractionAdd => process_effect_distraction_add(state),
        EffectKind::SetCostOverride {
            amount,
            only_reduce,
            random,
            scope,
        } => process_effect_set_cost_override(id_target, state, amount, only_reduce, random, scope),
        EffectKind::EscapePlanCheck { block } => process_effect_escape_plan_check(state, block),
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
        EffectKind::HealthSet { amount } => process_effect_health_set(id_target, state, amount),
        EffectKind::BlockGain { amount } => {
            process_effect_block_gain(id_source, id_target, state, amount)
        }
        EffectKind::BlockSet { amount } => process_effect_block_set(id_target, state, amount),
        EffectKind::EnergyDelta { sign, amount } => {
            process_effect_energy_delta(state, sign, amount)
        }
        EffectKind::ModifierGain { kind, stacks } => {
            process_effect_modifier_gain(id_target, state, kind, stacks)
        }
        EffectKind::ModifierMultiply { kind, factor } => {
            process_effect_modifier_multiply(id_target, state, kind, factor)
        }
        EffectKind::ModifierRemove { kind } => {
            process_effect_modifier_remove(id_target, state, kind)
        }
        EffectKind::ModifierTick => process_effect_modifier_tick(id_target, state),
        EffectKind::PoisonTick => process_effect_poison_tick(id_target, state),
        EffectKind::ModifierSetNotNew => process_effect_modifier_set_not_new(state),
        EffectKind::Death => {
            // Character can die outside Combat; empty monster slots make iter a no-op
            process_effect_death(id_target, state)
        }
        EffectKind::CombatStart => process_effect_combat_start(state),
        EffectKind::CombatEnd { escaped_character } => {
            process_effect_combat_end(state, escaped_character)
        }
        EffectKind::TurnStart => process_effect_turn_start(id_target, state),
        EffectKind::TurnEnd => process_effect_turn_end(id_target, state),
        EffectKind::MoveUpdate { move_override } => {
            process_effect_move_update(id_target, state, move_override)
        }
        EffectKind::MoveExecute => process_effect_move_execute(id_target, state),
        EffectKind::RoomEnter => process_effect_room_enter(state),
        EffectKind::MonsterSpawn { name, minion, cap } => {
            process_effect_monster_spawn(state, name, minion, cap)
        }
        EffectKind::MonsterSplit { name } => process_effect_monster_split(id_source, state, name),
        EffectKind::MonsterEscape => process_effect_monster_escape(id_target, state),
        EffectKind::GoldSteal { amount } => process_effect_gold_steal(id_source, state, amount),
        EffectKind::GremlinSummon => process_effect_gremlin_summon(state),
        EffectKind::DebuffsClear => process_effect_debuffs_clear(id_target, state),
        EffectKind::StasisSteal => process_effect_stasis_steal(id_source, state),
        EffectKind::JoustBet { on_owner } => process_effect_joust_bet(state, on_owner),
        EffectKind::KnowingSkullCostBump => {
            process_effect_knowing_skull_cost_bump(id_source, state)
        }
        EffectKind::MausoleumOpen => process_effect_mausoleum_open(state),
        EffectKind::HexaghostBurnIncrease { count } => {
            process_effect_hexaghost_burn_increase(state, count)
        }
        EffectKind::GoldDelta { sign, amount } => process_effect_gold_delta(state, sign, amount),
        EffectKind::RoomSelect => process_effect_room_select(id_target, state),
        EffectKind::CardPurge => process_effect_card_purge(id_target, state),
        EffectKind::CardDuplicate => process_effect_card_duplicate(id_target, state),
        EffectKind::CardTransform { upgraded } => {
            process_effect_card_transform(id_target, state, upgraded)
        }
        EffectKind::CardAdopt => process_effect_card_adopt(id_target, state),
        EffectKind::MaxHealthDelta { sign, amount } => {
            process_effect_max_health_delta(id_target, state, sign, amount)
        }
        EffectKind::ChestOpen => process_effect_chest_open(state),
        EffectKind::PotionDiscard => process_effect_potion_discard(id_target, state),
        EffectKind::ShopBuild => process_effect_shop_build(state),
        EffectKind::ShopBuy { slot } => process_effect_shop_buy(id_target, state, slot),
        EffectKind::ShopPurge => process_effect_shop_purge(state),
        EffectKind::PotionUse => process_effect_potion_use(id_target, state),
        EffectKind::PotionAddRandom { limited } => process_effect_potion_add_random(state, limited),
        EffectKind::PotionAdopt => process_effect_potion_adopt(id_target, state),
        EffectKind::CardDiscoverRoll {
            kind,
            color,
            exclude,
            count,
        } => {
            process_effect_card_discover_roll(state, kind, color, exclude, count);
        }
        EffectKind::Gamble {
            choose_discards,
            discards_before,
        } => process_effect_gamble(state, choose_discards, discards_before),
        EffectKind::RelicGrantPool { pool } => process_effect_relic_grant_pool(state, pool),
        EffectKind::RelicGrantRandom { tier } => process_effect_relic_grant_random(state, tier),
        EffectKind::RelicGrantSpecific {
            name,
            fallback_circlet,
        } => process_effect_relic_grant_specific(state, name, fallback_circlet),
        EffectKind::RelicLose => process_effect_relic_lose(id_target, state),
        EffectKind::RelicAdopt => process_effect_relic_adopt(id_target, state),
        EffectKind::EventAdvanceState { delta } => process_effect_event_advance_state(state, delta),
        EffectKind::ScrapOozeReach {
            dmg,
            chance,
            advance_on_miss,
        } => process_effect_scrap_ooze_reach(state, dmg, chance, advance_on_miss),
        EffectKind::EventConsume => process_effect_event_consume(state),
        EffectKind::CardDiscoverPick { cost_zero, pile } => {
            process_effect_card_discover_pick(id_target, state, cost_zero, pile)
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
            ensure_context_validity(state);
            return; // Queue drained
        };
        if !process_effect(state, effect) {
            ensure_context_validity(state);
            return; // Queue halted
        }
    }
}

// Cross-source witness: the active contexts must agree with each other and
// with world facts. Every active context is checked against the room directly
fn ensure_context_validity(state: &GameState) {
    if state.game_over {
        return;
    }
    let room_kind = get_active_room_kind(&state.id_rooms, state.location, &state.entities);

    // At most one room context owns the visit
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
    // chest, or rest site — only over an event (its fight) or the bare room
    assert!(
        !(state.combat.active && state.reward.active),
        "Combat and Reward both active"
    );
    assert!(
        !(state.combat.active
            && (state.shop.active || state.chest.active || state.rest_site.active)),
        "Combat active inside a non-event room context"
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

    // "?" rooms keep RoomKind::Unknown on the map after resolving
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
        // Neow rests over Location::Start, before any room exists
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
