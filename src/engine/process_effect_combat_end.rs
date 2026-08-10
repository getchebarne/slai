use crate::consts::ACT_FINAL;
use crate::consts::MAX_GOLD;
use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RewardSource;
use crate::effect::Target;
use crate::game::GameState;
use crate::game::Location;
use crate::map::get_active_room_kind;
use crate::modifier::modifier_clear;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::types::RelicName;
use crate::types::RoomKind;
use crate::utils::has_relic;
use crate::utils::mode_top_mut;
use crate::utils::roll_boss_gold;

pub fn process_effect_combat_end(state: &mut GameState, escaped_character: bool) {
    // Capture provenance, then drop the combat: teardown is the variant swap
    let mode = mode_top_mut(&mut state.mode_stack);
    let Mode::Combat {
        this_combat_escaped,
        event_loot,
        ..
    } = &*mode
    else {
        unreachable!("CombatEnd outside Combat mode")
    };
    let escaped_monster = *this_combat_escaped;
    let event_loot = *event_loot;
    *mode = Mode::CombatEnded;

    // Combat modifiers don't persist. Only the Character outlives the fight;
    // monster corpses are unreachable once the roster drops with the variant
    modifier_clear(&mut state.entities[state.id_character].modifiers);

    // Smoke Bomb: no rewards, no victory hooks; straight back to the map. Return
    // before the clear so already-queued effects (Toy Ornithopter heal) still land
    if escaped_character {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::RoomExit,
            id_source: None,
            target: Target::Direct(None),
        });
        return;
    }

    // Replace pending combat work with the teardown chain queued below
    state.effect_queue.clear();

    // A fight inside a live event belongs to the event (Colosseum's first bout):
    // pop back to it with no rewards; the post-combat relic hooks below still land
    if matches!(
        state.mode_stack.iter().rev().nth(1),
        Some(Mode::Event {
            consumed: false,
            ..
        })
    ) {
        state.mode_stack.pop();
    } else {
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
            RoomKind::CombatBoss
            | RoomKind::CombatMonster
            | RoomKind::CombatElite
            | RoomKind::Unknown
            | RoomKind::EventRoom => {
                // Stamped loot means an event started this fight (covers "?" rooms that
                // resolved to an event); otherwise a "?" marker is a normal monster combat
                let room_kind_reward = if event_loot.gold.is_some() {
                    RoomKind::EventRoom
                } else if room_kind == RoomKind::Unknown {
                    RoomKind::CombatMonster
                } else {
                    room_kind
                };
                state.effect_queue.push_back(Effect {
                    kind: EffectKind::RewardRoll {
                        source: RewardSource::Combat {
                            room_kind: room_kind_reward,
                            escaped: escaped_monster,
                            loot: event_loot,
                        },
                    },
                    id_source: None,
                    target: Target::Direct(None),
                });
            }
            RoomKind::RestSite | RoomKind::Treasure | RoomKind::Shop => {
                unreachable!("Combat end in non-combat room: {:?}", room_kind)
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

    // Final-act boss victory ends the run; the mode rests on CombatEnded (projected as Map)
    if matches!(state.location, Location::BossRoom) && state.act >= ACT_FINAL {
        state.game_over = true;
    }
}
