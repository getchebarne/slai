use crate::game::GameState;

// The event closes under the Reward it stages: leaving that Reward goes to the map
pub fn process_effect_event_close(state: &mut GameState) {
    assert!(state.event.active, "EventClose outside an event");
    state.event.active = false;
}
