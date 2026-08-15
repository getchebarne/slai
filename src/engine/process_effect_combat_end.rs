use crate::consts::ACT_FINAL;
use crate::consts::MAX_GOLD;
use crate::effect::Amount;
use crate::effect::EVENT_LOOT_ZERO;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RewardSource;
use crate::effect::Target;
use crate::events::fight_loot;
use crate::game::GameState;
use crate::game::Location;
use crate::map::get_active_room_kind;
use crate::modifier::modifier_clear;
use crate::types::DeltaSign;
use crate::types::Frame;
use crate::types::RelicName;
use crate::types::RoomKind;
use crate::utils::frame_top;
use crate::utils::frame_top_mut;
use crate::utils::has_relic;
use crate::utils::roll_boss_gold;

pub fn process_effect_combat_end(state: &mut GameState, escaped_character: bool) {
    let Frame::Combat {
        this_combat_escaped,
        ..
    } = frame_top(&state.frame_stack)
    else {
        unreachable!("CombatEnd outside the Combat frame")
    };
    let escaped_monster = *this_combat_escaped;

    // Clear the Character's modifiers
    modifier_clear(&mut state.entities[state.id_character].modifiers);

    // The spent Combat frame is consumed here; what it reveals owns the aftermath
    state.frame_stack.pop();

    // Smoke Bomb: no rewards, no victory hooks; already-queued effects
    // (Toy Ornithopter's heal) still land. A fled event fight spends its event
    if escaped_character {
        if let Frame::Event { consumed, .. } = frame_top_mut(&mut state.frame_stack) {
            *consumed = true;
        }
        return;
    }

    match frame_top_mut(&mut state.frame_stack) {
        // The fight belongs to the event it stacked over: staked loot pays out as
        // the reward; a lootless bout resumes the event (Colosseum's first)
        Frame::Event { kind, consumed, .. } => {
            if let Some(loot) = fight_loot(*kind) {
                *consumed = true;
                state.effect_queue.push_back(Effect {
                    kind: EffectKind::RewardRoll {
                        source: RewardSource::Combat {
                            room_kind: RoomKind::EventRoom,
                            escaped: escaped_monster,
                            loot,
                        },
                    },
                    id_source: None,
                    target: Target::Direct(None),
                });
            }
        }
        _ => {
            let room_kind =
                get_active_room_kind(&state.id_rooms, state.location, &state.entities).unwrap();
            match room_kind {
                // Final boss: gold granted directly, game_over halts the queue before
                // a GoldDelta would run
                RoomKind::CombatBoss if state.act >= ACT_FINAL => {
                    let amount = roll_boss_gold(&mut state.rng, state.ascension);

                    // Ectoplasm: no gold gain (roll still consumed for RNG parity with the source)
                    if !has_relic(&state.id_relics, RelicName::Ectoplasm) {
                        let gold = &mut state.entities[state.id_character].character_gold;
                        *gold = gold.saturating_add(amount).min(MAX_GOLD);
                    }
                }
                RoomKind::CombatBoss | RoomKind::CombatMonster | RoomKind::CombatElite => {
                    state.effect_queue.push_back(Effect {
                        kind: EffectKind::RewardRoll {
                            source: RewardSource::Combat {
                                room_kind,
                                escaped: escaped_monster,
                                loot: EVENT_LOOT_ZERO,
                            },
                        },
                        id_source: None,
                        target: Target::Direct(None),
                    });
                }
                // A "?" marker with no event frame beneath is a normal monster combat
                RoomKind::Unknown => {
                    state.effect_queue.push_back(Effect {
                        kind: EffectKind::RewardRoll {
                            source: RewardSource::Combat {
                                room_kind: RoomKind::CombatMonster,
                                escaped: escaped_monster,
                                loot: EVENT_LOOT_ZERO,
                            },
                        },
                        id_source: None,
                        target: Target::Direct(None),
                    });
                }
                RoomKind::RestSite | RoomKind::Treasure | RoomKind::Shop | RoomKind::EventRoom => {
                    unreachable!("Combat end in non-combat room: {:?}", room_kind)
                }
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
