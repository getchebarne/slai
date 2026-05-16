use crate::entity::Entity;
use crate::types::Phase;

pub fn process_effect_card_retain(id_card: usize, entities: &mut [Entity]) -> Option<Phase> {
    entities[id_card].card_retain = true;
    None
}
