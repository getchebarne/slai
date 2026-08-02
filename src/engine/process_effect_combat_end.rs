use rand::Rng;

use crate::consts::GOLD_BOSS_MAX;
use crate::consts::GOLD_BOSS_MIN;
use crate::consts::MAX_GOLD;
use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
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

pub fn process_effect_combat_end(state: &mut GameState, escaped_character: bool) {
    // Capture provenance, then drop the combat: teardown is the variant swap
    let mode = mode_top_mut(&mut state.mode_stack);
    let Mode::Combat {
        this_combat_escaped,
        event_gold,
        event_relic,
        event_relic_roll,
        ..
    } = &*mode
    else {
        unreachable!("CombatEnd outside Combat mode")
    };
    let escaped_monster = *this_combat_escaped;
    let event_gold = *event_gold;
    let event_relic = *event_relic;
    let event_relic_roll = *event_relic_roll;
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

    let room_kind = get_active_room_kind(&state.id_rooms, state.location, &state.entities).unwrap();
    match room_kind {
        RoomKind::CombatBoss => {
            // Granted directly: game_over halts the queue before a GoldDelta would run
            let roll = state.rng.random_range(GOLD_BOSS_MIN..=GOLD_BOSS_MAX);
            let amount = if state.ascension >= 13 {
                (roll * 3 + 2) / 4 // x0.75 rounded half-up
            } else {
                roll
            };

            // Ectoplasm: no gold gain (roll still consumed for RNG parity with the source game)
            if !has_relic(&state.id_relics, RelicName::Ectoplasm) {
                let gold = &mut state.entities[state.id_character].character_gold;
                *gold = gold.saturating_add(amount).min(MAX_GOLD);
            }
        }
        RoomKind::CombatMonster
        | RoomKind::CombatElite
        | RoomKind::Unknown
        | RoomKind::EventRoom => {
            // Stamped loot means an event started this fight (covers "?" rooms that
            // resolved to an event); otherwise a "?" marker is a normal monster combat
            let room_kind_reward = if event_gold.is_some() {
                RoomKind::EventRoom
            } else if room_kind == RoomKind::Unknown {
                RoomKind::CombatMonster
            } else {
                room_kind
            };
            state.effect_queue.push_back(Effect {
                kind: EffectKind::RewardRollCombat {
                    room_kind: room_kind_reward,
                    escaped: escaped_monster,
                    event_gold,
                    event_relic,
                    event_relic_roll,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }
        RoomKind::RestSite | RoomKind::Treasure | RoomKind::Shop => {
            unreachable!("Combat end in non-combat room: {:?}", room_kind)
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

    // Boss victory ends the run; the mode rests on CombatEnded (projected as Map)
    if matches!(state.location, Location::BossRoom) {
        state.game_over = true;
    }
}
