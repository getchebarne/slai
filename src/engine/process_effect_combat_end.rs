use rand::Rng;

use crate::consts::GOLD_BOSS_MAX;
use crate::consts::GOLD_BOSS_MIN;
use crate::consts::MAX_GOLD;
use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::game::Location;
use crate::map::get_active_room_kind;
use crate::modifier::modifier_clear;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::types::RelicName;
use crate::types::RoomKind;
use crate::utils::has_relic;

pub fn process_effect_combat_end(state: &mut GameState) {
    // Capture provenance, then drop the combat: teardown is the variant swap
    let Mode::Combat(combat) = &state.mode else {
        unreachable!("CombatEnd outside Combat mode")
    };
    let escaped = combat.this_combat_escaped;
    state.mode = Mode::CombatEnded;

    // Combat modifiers don't persist; arena entities live outside the variant
    for entity in state.entities.iter_mut() {
        if matches!(entity.kind, EntityKind::Monster | EntityKind::Character) {
            modifier_clear(&mut entity.modifiers);
        }
    }

    // Replace pending combat work with the teardown chain queued below
    state.effect_queue.clear();

    let room_kind = get_active_room_kind(&state.id_rooms, state.location, &state.entities).unwrap();
    match room_kind {
        RoomKind::CombatBoss => {
            // Granted directly: game_over halts the queue before a GoldDelta would run
            let roll = state.rng.random_range(GOLD_BOSS_MIN..=GOLD_BOSS_MAX);
            let amount = if state.ascension >= 13 {
                (roll * 3 + 2) / 4 // ×0.75 rounded half-up
            } else {
                roll
            };
            let gold = &mut state.entities[state.id_character].character_gold;
            *gold = gold.saturating_add(amount).min(MAX_GOLD);
        }
        RoomKind::CombatMonster
        | RoomKind::CombatElite
        | RoomKind::Unknown
        | RoomKind::EventRoom => {
            // A live event means an event started this fight (covers "?" rooms that
            // resolved to an event); otherwise a "?" marker is a normal monster combat
            let room_kind_reward = if state.event.is_some() {
                RoomKind::EventRoom
            } else if room_kind == RoomKind::Unknown {
                RoomKind::CombatMonster
            } else {
                room_kind
            };
            state.effect_queue.push_back(Effect {
                kind: EffectKind::RewardRollCombat {
                    room_kind: room_kind_reward,
                    escaped,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }
        RoomKind::RestSite | RoomKind::Treasure | RoomKind::Shop => {
            unreachable!("combat end in non-combat room: {:?}", room_kind)
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
