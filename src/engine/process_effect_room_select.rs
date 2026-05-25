use crate::effect::effect_direct;
use crate::effect::EffectKind;
use crate::game::GameState;
use crate::game::Location;

// Direct form (after resolver picked a target). Before resolution it's
// handled by the `Resolve` branch in `process_effect`
pub fn process_effect_room_select(id_target: Option<usize>, state: &mut GameState) {
    let id_room = id_target.expect("RoomSelect Direct form must have target");
    let room = &state.entities[id_room];
    state.location = Location::Overworld {
        y: room.room_y,
        x: room.room_x,
    };
    state
        .effect_queue
        .push_front(effect_direct(EffectKind::RoomEnter, None, None));
}
