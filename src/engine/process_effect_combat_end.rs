use crate::consts::MAX_MONSTERS;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::game::Location;
use crate::map::get_active_room_kind;
use crate::modifier::modifier_clear;
use crate::types::RoomKind;
use crate::types::Screen;

pub fn process_effect_combat_end(state: &mut GameState) {
    combat_reset(state);

    let room_kind = get_active_room_kind(&state.id_rooms, state.location, &state.entities).unwrap();
    match room_kind {
        RoomKind::CombatBoss => {}
        RoomKind::CombatMonster | RoomKind::CombatElite | RoomKind::Unknown => {
            // A "?" room keeps its Unknown map marker; its combat is always a normal monster
            let reward_kind = if room_kind == RoomKind::Unknown {
                RoomKind::CombatMonster
            } else {
                room_kind
            };
            state.effect_queue.push_back(Effect {
                kind: EffectKind::RewardRollCombat {
                    room_kind: reward_kind,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }
        RoomKind::RestSite | RoomKind::Treasure | RoomKind::EventRoom | RoomKind::Shop => {
            unreachable!("combat end in non-combat room: {:?}", room_kind)
        }
    }

    // BossRoom victory ends the run
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

    for entity in state.entities.iter_mut() {
        match entity.kind {
            EntityKind::Card => {
                entity.card_retain = false;
            }
            EntityKind::Monster | EntityKind::Character => {
                modifier_clear(&mut entity.modifiers);
            }
            _ => {}
        }
    }

    state.effect_queue.clear();
}
