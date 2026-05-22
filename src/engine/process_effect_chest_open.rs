use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::game::Location;
use crate::map::room_at_mut;

pub fn process_effect_chest_open(state: &mut GameState) {
    let Location::Overworld { y, x } = state.location else {
        panic!("ChestOpen outside Overworld");
    };

    let room = room_at_mut(&state.id_rooms, &mut state.entities, y, x)
        .expect("ChestOpen room missing");
    let chest_kind = room
        .room_chest_kind
        .expect("ChestOpen with no chest_kind on room");

    room.room_chest_opened = true;

    state.effect_queue.push_back(Effect {
        kind: EffectKind::RewardRollChest { kind: chest_kind },
        id_source: None,
        target: Target::Direct(None),
    });
}
