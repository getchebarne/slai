use std::collections::VecDeque;

use crate::consts::{MAP_HEIGHT, MAP_WIDTH};
use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::entity::{Entity, EntityKind};
use crate::map::active_room_kind;
use crate::modifier::modifier_clear;
use crate::state::Location;
use crate::types::RoomKind;

pub fn process_effect_combat_end(
    id_character: usize,
    id_hand: &mut Vec<usize>,
    id_pile_draw: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
    id_pile_exhaust: &mut Vec<usize>,
    id_card_target: &mut Option<usize>,
    entities: &mut Vec<Entity>,
    monster_count: &mut u8,
    id_rooms: &[[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    location: Location,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    id_hand.clear();
    id_pile_draw.clear();
    id_pile_discard.clear();
    id_pile_exhaust.clear();
    *id_card_target = None;

    let room = active_room_kind(id_rooms, location, entities).unwrap();

    modifier_clear(&mut entities[id_character].modifiers);
    for entity in entities.iter_mut() {
        match entity.kind {
            EntityKind::Card => {
                entity.card_retain = false;
            }
            EntityKind::Monster => {
                // Prevent stale Poison/Shackled/etc. from leaking into views
                // after the next combat reuses (or doesn't reuse) the slot.
                modifier_clear(&mut entity.modifiers);
            }
            _ => {}
        }
    }
    *monster_count = 0;
    match room {
        RoomKind::CombatBoss => {
            // Boss defeated — drop any pending effects. derive_resting_phase
            // returns GameOver from `location == BossRoom && monster_count == 0`.
            queue.clear();
        }
        RoomKind::CombatMonster => {
            queue.push_back(Effect {
                kind: EffectKind::CardRewardRoll,
                id_source: None,
                target: Target::Direct(None),
            });
        }
        RoomKind::RestSite => unreachable!("combat end in rest site"),
    }
    DispatchResult::Continue
}
