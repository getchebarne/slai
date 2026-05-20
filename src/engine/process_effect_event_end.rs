use crate::entity::Entity;

pub fn process_effect_event_end(entities: &mut [Entity], id_event: usize) {
    entities[id_event].event_consumed = true;
}
