use std::collections::VecDeque;

use crate::consts::MAP_HEIGHT;
use crate::consts::MAP_WIDTH;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::game::Location;
use crate::map::room_at_mut;
use crate::types::Phase;

pub fn process_effect_chest_open(
    id_rooms: &[[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    location: Location,
    entities: &mut Vec<Entity>,
    effect_queue: &mut VecDeque<Effect>,
) -> Option<Phase> {
    let Location::Overworld { y, x } = location else {
        panic!("ChestOpen outside Overworld");
    };

    let room = room_at_mut(id_rooms, entities, y, x).expect("ChestOpen room missing");
    let chest_kind = room
        .room_chest_kind
        .expect("ChestOpen with no chest_kind on room");

    room.room_chest_opened = true;

    effect_queue.push_back(Effect {
        kind: EffectKind::RewardRollChest { kind: chest_kind },
        id_source: None,
        target: Target::Direct(None),
    });

    None
}
