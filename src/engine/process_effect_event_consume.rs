use crate::game::GameState;

pub fn process_effect_event_consume(state: &mut GameState) {
    state
        .event
        .as_mut()
        .expect("EventConsume without an active event")
        .consumed = true;
}
