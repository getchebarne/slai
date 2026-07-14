use rand::Rng;

use crate::consts::GOLD_BOSS_MAX;
use crate::consts::GOLD_BOSS_MIN;
use crate::consts::MAX_GOLD;
use crate::consts::MAX_MONSTERS;
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
use crate::types::RelicName;
use crate::types::RoomKind;
use crate::types::Screen;
use crate::utils::has_relic;

pub fn process_effect_combat_end(state: &mut GameState) {
    combat_reset(state);

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
            // A live id_event means an event started this fight (covers "?" rooms that
            // resolved to an event); otherwise a "?" marker is a normal monster combat
            let room_kind_reward = if state.id_event.is_some() {
                RoomKind::EventRoom
            } else if room_kind == RoomKind::Unknown {
                RoomKind::CombatMonster
            } else {
                room_kind
            };
            state.effect_queue.push_back(Effect {
                kind: EffectKind::RewardRollCombat {
                    room_kind: room_kind_reward,
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

    // Boss victory ends the run
    if matches!(state.location, Location::BossRoom) {
        state.screen = Screen::Map;
        state.game_over = true;
    }
}

fn combat_reset(state: &mut GameState) {
    state.id_hand.clear();
    state.id_pile_draw.clear();
    state.id_pile_discard.clear();
    state.id_pile_exhaust.clear();
    state.id_discover.clear();
    state.id_card_nightmare = None;
    state.id_picked_monster = None;
    state.id_monsters = [None; MAX_MONSTERS];
    state.this_turn_discards = 0;
    state.this_turn_attacks = 0;
    state.this_combat_damage_instances_taken = 0;
    state.this_combat_escaped = false;
    state.id_card_last_drawn = None;

    // Combat modifiers don't persist; card combat-state lives on the discarded copies
    for entity in state.entities.iter_mut() {
        if matches!(entity.kind, EntityKind::Monster | EntityKind::Character) {
            modifier_clear(&mut entity.modifiers);
        }
    }

    state.effect_queue.clear();
}
