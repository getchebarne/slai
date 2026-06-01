use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::game::Location;

// Direct form (post-resolve); Resolve form handled by process_effect
pub fn process_effect_room_select(id_target: Option<usize>, state: &mut GameState) {
    let id_room = id_target.expect("RoomSelect Direct form must have target");
    let room = &state.entities[id_room];
    state.location = Location::Overworld {
        y: room.room_y,
        x: room.room_x,
    };
    state.effect_queue.push_front(Effect {
        kind: EffectKind::RoomEnter,
        id_source: None,
        target: Target::Direct(None),
    });
}
