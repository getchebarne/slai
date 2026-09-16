use crate::consts::ACT_FINAL;
use crate::consts::BOSS_RELIC_REWARD_COUNT;
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
use crate::relics::iter_owned_relics;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::types::RelicTier;
use crate::types::RoomKind;
use crate::types::reward_reset;
use crate::utils::has_relic;
use crate::utils::queue_effect_untargeted;
use crate::utils::roll_boss_gold;

pub fn process_effect_combat_end(state: &mut GameState, escaped_character: bool) {
    assert!(state.combat.active, "CombatEnd outside combat");
    let escaped_monster = state.combat.this_combat_escaped;

    // Clear the Character's modifiers and block: resetPlayer wipes both
    modifier_clear(&mut state.entities[state.id_character].modifiers);
    state.entities[state.id_character].vitals.block = 0;

    // The spent combat is closed here; what it reveals owns the aftermath
    state.combat.active = false;

    // Smoke Bomb: no rewards; endBattle still runs, so addPotionToRewards drifts the drop
    // chance and the victory Relics still fire
    if escaped_character {
        queue_effect_untargeted(
            state,
            EffectKind::RewardRollPotion {
                eligible: true,
                staged: false,
            },
        );
        queue_combat_end_relics(state);
        if state.event.active {
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
                EffectKind::RewardRollPotion {
                    eligible: true,
                    staged: false,
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
            for pick in loot.relics.into_iter().flatten() {
                queue_effect_untargeted(
                    state,
                    EffectKind::RewardRollRelic {
                        pick,
                        exclusion: RelicExclusion::Unfiltered,
                    },
                );
            }
            queue_effect_untargeted(
                state,
                EffectKind::RewardRollPotion {
                    eligible: true,
                    staged: true,
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

            // The Reward context opens up front so the boss flag rides the reset
            reward_reset(&mut state.reward);
            state.reward.relics_exclusive = room_kind == RoomKind::CombatBoss;
            state.reward.active = true;

            // The thieves' purse is its own reward item, without the Idol bonus
            if state.combat.gold_stolen > 0 {
                state.reward.gold = Some(state.combat.gold_stolen);
            }

            // Boss rewards draw from the rare pool only; Elites widen both bands
            queue_effect_untargeted(
                state,
                EffectKind::RewardRollCards {
                    bundles,
                    trigger: match room_kind {
                        RoomKind::CombatBoss => RewardRollTrigger::CombatBoss,
                        RoomKind::CombatElite => RewardRollTrigger::CombatElite,
                        _ => RewardRollTrigger::CombatMonster,
                    },
                },
            );

            // The boss offers three unique unowned Boss Relics; RewardTake keeps one
            if room_kind == RoomKind::CombatBoss {
                for _ in 0..BOSS_RELIC_REWARD_COUNT {
                    queue_effect_untargeted(
                        state,
                        EffectKind::RewardRollRelic {
                            pick: RelicPick::Tier(RelicTier::Boss),
                            exclusion: RelicExclusion::Unfiltered,
                        },
                    );
                }
            }

            // Elite drop; Black Star adds a second with an independent tier roll
            if room_kind == RoomKind::CombatElite {
                let pick = RelicPick::Thresholds {
                    th_common: RELIC_TIER_TH_COMMON,
                    th_uncommon: RELIC_TIER_TH_UNCOMMON,
                };
                queue_effect_untargeted(
                    state,
                    EffectKind::RewardRollRelic {
                        pick,
                        exclusion: RelicExclusion::Unfiltered,
                    },
                );
                if has_relic(&state.id_relics, RelicName::BlackStar) {
                    queue_effect_untargeted(
                        state,
                        EffectKind::RewardRollRelic {
                            pick,
                            exclusion: RelicExclusion::NonCampfire,
                        },
                    );
                }
            }

            // Escaped normal fights roll Potion chance 0 in the source
            queue_effect_untargeted(
                state,
                EffectKind::RewardRollPotion {
                    eligible: !(room_kind == RoomKind::CombatMonster && escaped_monster),
                    staged: true,
                },
            );

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

// endBattle's Relic hooks: they fire on a victory and on a Smoke Bomb alike
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
