use crate::entity::Entity;
use crate::types::Phase;

pub fn process_effect_event_end(entities: &mut [Entity], id_event: usize) -> Option<Phase> {
    entities[id_event].event_consumed = true;
    None
}
