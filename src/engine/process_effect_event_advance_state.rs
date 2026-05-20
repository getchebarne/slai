use crate::entity::Entity;

pub fn process_effect_event_advance_state(
    entities: &mut [Entity],
    id_event: usize,
    delta: i8,
) {
    let event = &mut entities[id_event];
    let new_state = (event.event_state as i16 + delta as i16).max(0);
    event.event_state = new_state.min(u8::MAX as i16) as u8;
}
