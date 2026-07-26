pub mod process_effect_adventurer_search;
pub mod process_effect_block_gain;
pub mod process_effect_block_set;
pub mod process_effect_bonfire_offer;
pub mod process_effect_calculated_gamble;
pub mod process_effect_card_add;
pub mod process_effect_card_add_random;
pub mod process_effect_card_adopt;
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
pub mod process_effect_distraction_add;
pub mod process_effect_energy_delta;
pub mod process_effect_escape_plan_check;
pub mod process_effect_event_advance_state;
pub mod process_effect_event_consume;
pub mod process_effect_face_trade;
pub mod process_effect_glass_knife_decay;
pub mod process_effect_gold_delta;
pub mod process_effect_gold_steal;
pub mod process_effect_hand_of_greed_proc;
pub mod process_effect_health_delta;
pub mod process_effect_health_set;
pub mod process_effect_heel_hook_proc;
pub mod process_effect_hexaghost_burn_increase;
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
pub mod process_effect_relic_grant_random;
pub mod process_effect_relic_grant_specific;
pub mod process_effect_rest_site_consume;
pub mod process_effect_reward_roll_chest;
pub mod process_effect_reward_roll_combat;
pub mod process_effect_reward_roll_potions;
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
pub mod process_effect_wheel_spin;

use std::collections::VecDeque;

use rand::Rng;

use crate::effect::CandidatePool;
use crate::effect::CandidatePoolMonstersFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::events::EventKind;
use crate::game::GameState;
use crate::game::Location;
use crate::map::get_active_room_kind;
use crate::map::room_at;
use crate::types::CardColor;
use crate::types::Mode;
use crate::types::RoomKind;
use crate::utils::card_filter_matches;
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
    mode: &Mode,
    entities: &[Entity],
    id_deck: &[usize],
) {
    // Combat-scoped pools demand the combat context; Character/Source/Deck don't
    match candidate_pool {
        CandidatePool::Hand { filter } => {
            let Mode::Combat { id_hand, .. } = mode else {
                unreachable!("Hand pool outside Combat mode")
            };
            for &id in id_hand {
                if card_filter_matches(filter, &entities[id]) {
                    effect_candidate_buf.push(id);
                }
            }
        }
        CandidatePool::PileDraw { filter } => {
            let Mode::Combat { id_pile_draw, .. } = mode else {
                unreachable!("PileDraw pool outside Combat mode")
            };
            for &id in id_pile_draw {
                if card_filter_matches(filter, &entities[id]) {
                    effect_candidate_buf.push(id);
                }
            }
        }
        CandidatePool::PileDiscard => {
            let Mode::Combat {
                id_pile_discard, ..
            } = mode
            else {
                unreachable!("PileDiscard pool outside Combat mode")
            };
            effect_candidate_buf.extend_from_slice(id_pile_discard)
        }
        CandidatePool::PileExhaust => {
            let Mode::Combat {
                id_pile_exhaust, ..
            } = mode
            else {
                unreachable!("PileExhaust pool outside Combat mode")
            };
            effect_candidate_buf.extend_from_slice(id_pile_exhaust)
        }
        CandidatePool::Character => effect_candidate_buf.push(id_character),
        CandidatePool::Monsters { filter } => {
            let Mode::Combat {
                id_monsters,
                id_picked_monster,
                ..
            } = mode
            else {
                unreachable!("Monsters pool outside Combat mode")
            };
            match filter {
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
            }
        }
        CandidatePool::Source => {
            let id_source = id_source
                .expect("Attempted to resolve `CandidatePool::Source` without `id_source`");

            effect_candidate_buf.push(id_source)
        }
        CandidatePool::Discover => {
            let Mode::Combat { id_discover, .. } = mode else {
                unreachable!("Discover pool outside Combat mode")
            };
            effect_candidate_buf.extend_from_slice(id_discover)
        }
        CandidatePool::Deck { filter } => {
            for &id in id_deck {
                if card_filter_matches(filter, &entities[id]) {
                    effect_candidate_buf.push(id);
                }
            }
        }
        CandidatePool::EventPickCard => {
            let Mode::Event {
                kind: EventKind::WeMeetAgain { id_card, .. },
                ..
            } = mode
            else {
                unreachable!("EventPickCard pool outside We Meet Again")
            };
            effect_candidate_buf.push(id_card.expect("EventPickCard without a rolled pick"));
        }
        CandidatePool::EventPickPotion => {
            let Mode::Event {
                kind: EventKind::WeMeetAgain { id_potion, .. },
                ..
            } = mode
            else {
                unreachable!("EventPickPotion pool outside We Meet Again")
            };
            effect_candidate_buf.push(id_potion.expect("EventPickPotion without a rolled pick"));
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
        &state.mode,
        &state.entities,
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
        EffectKind::CardDrawIfNoAttacks { count } => {
            process_effect_card_draw_if_no_attacks::process_effect_card_draw_if_no_attacks(
                state, count,
            )
        }
        EffectKind::CardAddRandom {
            color,
            kind,
            pile,
            count,
            cost_zero,
            upgraded,
        } => process_effect_card_add_random::process_effect_card_add_random(
            state, color, kind, pile, count, cost_zero, upgraded,
        ),
        EffectKind::HandOfGreedProc { gold } => {
            process_effect_hand_of_greed_proc::process_effect_hand_of_greed_proc(
                id_target, state, gold,
            )
        }
        EffectKind::CardDrawUpTo { amount } => {
            process_effect_card_draw_up_to::process_effect_card_draw_up_to(state, amount)
        }
        EffectKind::CardPlay => {
            process_effect_card_play::process_effect_card_play(id_target, state)
        }
        EffectKind::CardAdd {
            card_name,
            pile,
            count,
            upgraded,
        } => process_effect_card_add::process_effect_card_add(state, card_name, pile, count, upgraded),
        EffectKind::CardDiscard { source } => {
            process_effect_card_discard::process_effect_card_discard(id_target, state, source)
        }
        EffectKind::CardMove { pile } => {
            process_effect_card_move::process_effect_card_move(id_target, state, pile)
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
        EffectKind::CardSetupPick { free, bottom } => {
            process_effect_card_setup_pick::process_effect_card_setup_pick(
                id_target, state, free, bottom,
            )
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
        EffectKind::CardPlayFromDrawTop => {
            process_effect_card_play_from_draw_top::process_effect_card_play_from_draw_top(state)
        }
        EffectKind::CardRemove => {
            process_effect_card_remove::process_effect_card_remove(id_target, state)
        }
        EffectKind::CalculatedGamble => {
            process_effect_calculated_gamble::process_effect_calculated_gamble(state)
        }
        EffectKind::AdventurerSearch => {
            process_effect_adventurer_search::process_effect_adventurer_search(state)
        }
        EffectKind::BonfireOffer => {
            process_effect_bonfire_offer::process_effect_bonfire_offer(id_target, state)
        }
        EffectKind::FaceTrade => process_effect_face_trade::process_effect_face_trade(state),
        EffectKind::WheelSpin => process_effect_wheel_spin::process_effect_wheel_spin(state),
        EffectKind::CardUpgrade => {
            process_effect_card_upgrade::process_effect_card_upgrade(id_target, state)
        }
        EffectKind::RewardRollCombat {
            room_kind,
            escaped,
            event_gold,
            event_relic,
            event_relic_roll,
        } => process_effect_reward_roll_combat::process_effect_reward_roll_combat(
            state,
            room_kind,
            escaped,
            event_gold,
            event_relic,
            event_relic_roll,
        ),
        EffectKind::RewardRollPotions { count } => {
            process_effect_reward_roll_potions::process_effect_reward_roll_potions(state, count)
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
        EffectKind::SetCostOverride {
            amount,
            only_reduce,
            scope,
        } => process_effect_set_cost_override::process_effect_set_cost_override(
            id_target,
            state,
            amount,
            only_reduce,
            scope,
        ),
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
        EffectKind::HealthSet { amount } => {
            process_effect_health_set::process_effect_health_set(id_target, state, amount)
        }
        EffectKind::BlockGain { amount } => process_effect_block_gain::process_effect_block_gain(
            id_source, id_target, state, amount,
        ),
        EffectKind::BlockSet { amount } => {
            process_effect_block_set::process_effect_block_set(id_target, state, amount)
        }
        EffectKind::EnergyDelta { sign, amount } => {
            process_effect_energy_delta::process_effect_energy_delta(state, sign, amount)
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
        EffectKind::CombatStart {
            event_gold,
            event_relic,
            event_relic_roll,
        } => process_effect_combat_start::process_effect_combat_start(
            state,
            event_gold,
            event_relic,
            event_relic_roll,
        ),
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
        EffectKind::MoveUpdate { move_override } => {
            process_effect_move_update::process_effect_move_update(id_target, state, move_override)
        }
        EffectKind::MoveExecute => {
            process_effect_move_execute::process_effect_move_execute(id_target, state)
        }
        EffectKind::RoomEnter => process_effect_room_enter::process_effect_room_enter(state),
        EffectKind::MonsterSpawn { name } => {
            process_effect_monster_spawn::process_effect_monster_spawn(state, name)
        }
        EffectKind::MonsterSplit { name } => {
            process_effect_monster_split::process_effect_monster_split(id_source, state, name)
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
            process_effect_event_advance_state::process_effect_event_advance_state(state, delta)
        }
        EffectKind::ScrapOozeReach {
            dmg,
            chance,
            advance_on_miss,
        } => process_effect_scrap_ooze_reach::process_effect_scrap_ooze_reach(
            state,
            dmg,
            chance,
            advance_on_miss,
        ),
        EffectKind::EventConsume => {
            process_effect_event_consume::process_effect_event_consume(state)
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
            ensure_mode_validity(state);
            return; // Queue drained
        };
        if !process_effect(state, effect) {
            ensure_mode_validity(state);
            return; // Queue halted
        }
    }
}

// Cross-source witness: the mode must agree with world facts (room kind, room
// flags, event presence) at every rest. A stale mode cannot lie to all of them
fn ensure_mode_validity(state: &GameState) {
    if state.game_over {
        return;
    }
    let room_kind = get_active_room_kind(&state.id_rooms, state.location, &state.entities);
    let room = match state.location {
        Location::Overworld { y, x } => room_at(&state.id_rooms, &state.entities, y, x),
        _ => None,
    };
    let ok = match &state.mode {
        // Map doubles as the between-rooms state; nothing to cross-check
        Mode::Map => true,
        Mode::RestSite => room_kind == Some(RoomKind::RestSite),
        Mode::Chest => {
            matches!(room_kind, Some(RoomKind::Treasure | RoomKind::Unknown))
                && room.is_some_and(|room| !room.room_chest_opened)
        }
        Mode::ChestOpened => {
            matches!(room_kind, Some(RoomKind::Treasure | RoomKind::Unknown))
                && room.is_some_and(|room| room.room_chest_opened)
        }
        Mode::Event { .. } => matches!(room_kind, Some(RoomKind::EventRoom | RoomKind::Unknown)),
        // "?" rooms keep RoomKind::Unknown on the map after resolving
        Mode::Shop { .. } => matches!(room_kind, Some(RoomKind::Shop | RoomKind::Unknown)),
        Mode::Combat { .. } | Mode::CombatEnded => matches!(
            room_kind,
            Some(
                RoomKind::CombatMonster
                    | RoomKind::CombatElite
                    | RoomKind::CombatBoss
                    | RoomKind::EventRoom
                    | RoomKind::Unknown
            )
        ),
        Mode::Reward { .. } => matches!(
            room_kind,
            Some(
                RoomKind::CombatMonster
                    | RoomKind::CombatElite
                    | RoomKind::EventRoom
                    | RoomKind::Treasure
                    | RoomKind::Unknown
            )
        ),
    };
    assert!(
        ok,
        "mode {:?} inconsistent with room kind {:?} at {:?}",
        state.mode, room_kind, state.location
    );
}
