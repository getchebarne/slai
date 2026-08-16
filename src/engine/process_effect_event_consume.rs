use crate::game::GameState;

pub fn process_effect_event_consume(state: &mut GameState) {
    assert!(state.event.active, "EventConsume outside an event");
    state.event.consumed = true;
}
