use crate::game::GameState;

// The event closes without a RoomExit: the map shows once nothing it staged is still open
pub fn process_effect_event_close(state: &mut GameState) {
    assert!(state.event.active, "EventClose outside an event");
    state.event.active = false;
}
