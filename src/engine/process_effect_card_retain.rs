use crate::entity::Entity;

pub fn process_effect_card_retain(id_card: usize, entities: &mut [Entity]) {
    entities[id_card].card_retain = true;
}
