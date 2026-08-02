use crate::consts::MAP_HEIGHT;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::game::Location;
use crate::map::get_active_room_kind;
use crate::types::RoomKind;

pub fn process_effect_room_exit(state: &mut GameState) {
    // Pop the room frame; Map (or a suspended frame) resumes underneath
    assert!(
        state.mode_stack.len() > 1,
        "RoomExit with no room frame to pop"
    );
    state.mode_stack.pop();

    // final-row rest room enters the boss instead of returning to the map; keyed on
    // the room, not the popped frame (Dream Catcher's reward replaces RestSite)
    if matches!(state.location, Location::Overworld { y, .. } if y == MAP_HEIGHT - 1)
        && get_active_room_kind(&state.id_rooms, state.location, &state.entities)
            == Some(RoomKind::RestSite)
    {
        state.location = Location::BossRoom;
        state.effect_queue.push_front(Effect {
            kind: EffectKind::RoomEnter,
            id_source: None,
            target: Target::Direct(None),
        });
    }
}
