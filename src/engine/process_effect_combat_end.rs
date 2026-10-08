use rand::Rng;

use crate::consts::ACT_FINAL;
use crate::consts::GOLD_ELITE_MAX;
use crate::consts::GOLD_ELITE_MIN;
use crate::consts::GOLD_MONSTER_MAX;
use crate::consts::GOLD_MONSTER_MIN;
use crate::consts::RELIC_TIER_TH_COMMON;
use crate::consts::RELIC_TIER_TH_UNCOMMON;
use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RelicExclusion;
use crate::effect::RelicPick;
use crate::effect::RewardRollTrigger;
use crate::effect::Target;
use crate::events::fight_loot;
use crate::game::GameState;
use crate::game::Location;
use crate::map::get_active_room_kind;
use crate::modifier::modifier_clear;
use crate::relics::RELIC_COUNTERS_COMBAT_ONLY;
use crate::relics::iter_owned_relics;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::types::RoomKind;
use crate::types::reward_reset;
use crate::utils::draw_relic_excluding;
use crate::utils::has_relic;
use crate::utils::queue_effect_untargeted;
use crate::utils::relic_tier_by_roll;
use crate::utils::roll_boss_gold;

pub fn process_effect_combat_end(state: &mut GameState, escaped_character: bool) {
    assert!(state.combat.active, "CombatEnd outside combat");
    let escaped_monster = state.combat.this_combat_escaped;
    let thief_escaped = state.combat.this_combat_thief_escaped;

    // Clear the Character's modifiers and block
    modifier_clear(&mut state.entities[state.id_character].modifiers);
    state.entities[state.id_character].vitals.block = 0;

    // Combat-only Relic counters hide until the next combat
    for &name in RELIC_COUNTERS_COMBAT_ONLY {
        if let Some(id) = state.id_relics[name as usize] {
            state.entities[id].relic_counter = -1;
        }
    }

    // Card plays still waiting never start
    state.card_play_queue.clear();

    // The spent combat is closed here; what it reveals owns the aftermath
    state.combat.active = false;

    // Smoke Bomb: no rewards, unless a thief got away first
    if escaped_character && !thief_escaped {
        // The Relics the fight drops are still drawn, and lost for the run
        for (pick, exclusion) in relic_drops(state) {
            let tier = match pick {
                RelicPick::Thresholds {
                    th_common,
                    th_uncommon,
                } => {
                    relic_tier_by_roll(state.rng.random_range(0..100) as u8, th_common, th_uncommon)
                }
                RelicPick::Tier(tier) => tier,

                // A named or pool pick takes nothing from the tier pools
                RelicPick::Pool(_) | RelicPick::Name(_) => continue,
            };
            draw_relic_excluding(state, tier, exclusion);
        }

        // The Potion drop chance still drifts and the combat-end Relics still fire
        queue_effect_untargeted(
            state,
            EffectKind::RewardRollPotions {
                count: 1,
                trigger: RewardRollTrigger::SmokeBomb,
            },
        );
        queue_combat_end_relics(state);

        // A fled event fight spends its event; an unpaid bout resumes it as a won one does
        if state.event.active && fight_loot(&state.event).is_some() {
            state.event.consumed = true;
        }
        return;
    }

    // Queue order is RNG stream order: Cards, Relics, Potion, then gold
    if state.event.active {
        // Colosseum's unpaid bout: the potion roll drifts, the stage is cleared
        if fight_loot(&state.event).is_none() {
            queue_effect_untargeted(
                state,
                EffectKind::RewardRollPotions {
                    count: 1,
                    trigger: RewardRollTrigger::EventFightUnpaid,
                },
            );
        }

        // The fight belongs to the event it stacked over
        if let Some(loot) = fight_loot(&state.event) {
            state.event.consumed = true;
            queue_effect_untargeted(
                state,
                EffectKind::RewardRollCards {
                    bundles: 1,
                    trigger: RewardRollTrigger::EventFight,
                },
            );
            for (pick, exclusion) in relic_drops(state) {
                queue_effect_untargeted(state, EffectKind::RewardRollRelic { pick, exclusion });
            }
            queue_effect_untargeted(
                state,
                EffectKind::RewardRollPotions {
                    count: 1,
                    trigger: RewardRollTrigger::EventFight,
                },
            );
            if let Some(amount) = loot.gold {
                queue_effect_untargeted(state, EffectKind::RewardRollGold { amount });
            }
        }
    } else {
        // Final boss: the run ends below; every other fight rolls its reward
        if !(matches!(state.location, Location::BossRoom) && state.act >= ACT_FINAL) {
            // A "?" that resolved to a plain fight rewards as a normal Monster Room
            let room_kind =
                match get_active_room_kind(&state.id_rooms, state.location, &state.entities)
                    .expect("Combat reward outside any room")
                {
                    RoomKind::Unknown => RoomKind::CombatMonster,
                    kind => kind,
                };

            // Boss gold pre-rolls here so Golden Idol still scales it at staging
            let gold_amount = match room_kind {
                RoomKind::CombatMonster => (!escaped_monster).then_some(Amount::Range {
                    min: GOLD_MONSTER_MIN,
                    max: GOLD_MONSTER_MAX,
                }),
                RoomKind::CombatElite => Some(Amount::Range {
                    min: GOLD_ELITE_MIN,
                    max: GOLD_ELITE_MAX,
                }),
                RoomKind::CombatBoss => Some(Amount::Absolute(roll_boss_gold(
                    &mut state.rng,
                    state.ascension,
                ))),
                _ => unreachable!("CombatEnd in a non-combat room: {room_kind:?}"),
            };

            // Prayer Wheel: adds a second Card bundle on normal fights
            let bundles = if room_kind == RoomKind::CombatMonster
                && has_relic(&state.id_relics, RelicName::PrayerWheel)
            {
                2
            } else {
                1
            };

            // The Reward context opens up front; the boss's Relics wait in its treasure Room
            reward_reset(&mut state.reward);
            state.reward.active = true;

            // The thieves' purse is its own reward item, without Golden Idol's bonus
            if state.combat.gold_stolen > 0 {
                state.reward.gold_stolen = Some(state.combat.gold_stolen);
            }

            // Boss rewards draw from the rare pool only; Elites widen both bands
            let trigger = match room_kind {
                RoomKind::CombatBoss => RewardRollTrigger::CombatBoss,
                RoomKind::CombatElite => RewardRollTrigger::CombatElite,
                _ => RewardRollTrigger::CombatMonster,
            };
            queue_effect_untargeted(state, EffectKind::RewardRollCards { bundles, trigger });
            for (pick, exclusion) in relic_drops(state) {
                queue_effect_untargeted(state, EffectKind::RewardRollRelic { pick, exclusion });
            }
            queue_effect_untargeted(state, EffectKind::RewardRollPotions { count: 1, trigger });

            if let Some(amount) = gold_amount {
                queue_effect_untargeted(state, EffectKind::RewardRollGold { amount });
            }
        }
    }

    queue_combat_end_relics(state);

    // Final-act boss victory ends the run, resting on Map
    if matches!(state.location, Location::BossRoom) && state.act >= ACT_FINAL {
        state.game_over = true;
    }
}

// The Relics a fight drops: its event's staked loot, or an Elite's
fn relic_drops(state: &GameState) -> Vec<(RelicPick, RelicExclusion)> {
    if state.event.active {
        let Some(loot) = fight_loot(&state.event) else {
            return Vec::new();
        };
        return loot
            .relics
            .into_iter()
            .flatten()
            .map(|pick| (pick, RelicExclusion::Unfiltered))
            .collect();
    }
    if get_active_room_kind(&state.id_rooms, state.location, &state.entities)
        != Some(RoomKind::CombatElite)
    {
        return Vec::new();
    }
    let pick = RelicPick::Thresholds {
        th_common: RELIC_TIER_TH_COMMON,
        th_uncommon: RELIC_TIER_TH_UNCOMMON,
    };
    let mut drops = vec![(pick, RelicExclusion::Unfiltered)];

    // Black Star: a second drop with an independent tier roll
    if has_relic(&state.id_relics, RelicName::BlackStar) {
        drops.push((pick, RelicExclusion::NonCampfire));
    }
    drops
}

// Combat-end Relic hooks: they fire on a victory and on a Smoke Bomb alike
fn queue_combat_end_relics(state: &mut GameState) {
    // Meat on the Bone: ending combat at half HP or less heals 12
    if has_relic(&state.id_relics, RelicName::MeatOnTheBone) {
        let vitals = &state.entities[state.id_character].vitals;
        if vitals.health > 0 && vitals.health * 2 <= vitals.health_max {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::HealthDelta {
                    sign: DeltaSign::Gain,
                    amount: Amount::Absolute(12),
                },
                id_source: None,
                target: Target::Direct(Some(state.id_character)),
            });
        }
    }

    // Combat-end Relic effects, in acquisition order (Face of Cleric, etc.)
    let mut id_relics: Vec<usize> = iter_owned_relics(&state.id_relics)
        .map(|(_, id)| id)
        .collect();
    id_relics.sort_unstable_by_key(|&id| state.entities[id].relic_seq);

    for id_relic in id_relics {
        for &effect in state.entities[id_relic].relic_effects_combat_end {
            state.effect_queue.push_back(effect);
        }
    }
}
