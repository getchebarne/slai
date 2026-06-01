use crate::game::GameState;

pub fn process_effect_event_advance_state(
    id_source: Option<usize>,
    state: &mut GameState,
    delta: i8,
) {
    let id_event = id_source.expect("EventAdvanceState requires id_source");
    let event = &mut state.entities[id_event];
    let new_state = (event.event_state as i16 + delta as i16).max(0);
    event.event_state = new_state.min(u8::MAX as i16) as u8;
}
