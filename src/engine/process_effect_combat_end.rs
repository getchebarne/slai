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
use crate::effect::RelicPick;
use crate::effect::Target;
use crate::events::fight_loot;
use crate::game::GameState;
use crate::game::Location;
use crate::map::get_active_room_kind;
use crate::modifier::modifier_clear;
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

    // Clear the Character's modifiers
    modifier_clear(&mut state.entities[state.id_character].modifiers);

    // The spent combat is closed here; what it reveals owns the aftermath
    state.combat.active = false;

    // Smoke Bomb: no rewards, no victory hooks
    if escaped_character {
        if state.event.active {
            state.event.consumed = true;
        }
        return;
    }

    // Queue order is RNG stream order: cards, relics, potion, then gold
    if state.event.active {
        // The fight belongs to the event it stacked over
        if let Some(loot) = fight_loot(&state.event) {
            state.event.consumed = true;
            queue_effect_untargeted(
                state,
                EffectKind::RewardRollCards {
                    bundles: 1,
                    rare_only: false,
                },
            );
            for pick in loot.relics.into_iter().flatten() {
                queue_effect_untargeted(state, EffectKind::RewardRollRelic { pick });
            }
            queue_effect_untargeted(state, EffectKind::RewardRollPotion { eligible: true });
            if let Some(amount) = loot.gold {
                queue_effect_untargeted(state, EffectKind::RewardRollGold { amount });
            }
        }
    } else {
        // Final boss: the run ends below; every other fight rolls its reward
        if !(matches!(state.location, Location::BossRoom) && state.act >= ACT_FINAL) {
            // A "?" that resolved to a plain fight rewards as a normal monster room
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

            // Prayer Wheel: adds a second card bundle on normal fights
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

            // Boss rewards draw from the rare pool only
            queue_effect_untargeted(
                state,
                EffectKind::RewardRollCards {
                    bundles,
                    rare_only: room_kind == RoomKind::CombatBoss,
                },
            );

            // The boss offers three unique unowned Boss relics; RewardTake keeps one
            if room_kind == RoomKind::CombatBoss {
                for _ in 0..BOSS_RELIC_REWARD_COUNT {
                    queue_effect_untargeted(
                        state,
                        EffectKind::RewardRollRelic {
                            pick: RelicPick::Tier(RelicTier::Boss),
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
                queue_effect_untargeted(state, EffectKind::RewardRollRelic { pick });
                if has_relic(&state.id_relics, RelicName::BlackStar) {
                    queue_effect_untargeted(state, EffectKind::RewardRollRelic { pick });
                }
            }

            // Escaped normal fights roll potion chance 0 in the source
            queue_effect_untargeted(
                state,
                EffectKind::RewardRollPotion {
                    eligible: !(room_kind == RoomKind::CombatMonster && escaped_monster),
                },
            );

            if let Some(amount) = gold_amount {
                queue_effect_untargeted(state, EffectKind::RewardRollGold { amount });
            }
        }
    }

    // Face of Cleric: +1 max HP after each combat
    if has_relic(&state.id_relics, RelicName::FaceOfCleric) {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::MaxHealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(1),
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

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

    // Final-act boss victory ends the run, resting on Map
    if matches!(state.location, Location::BossRoom) && state.act >= ACT_FINAL {
        state.game_over = true;
    }
}
