use crate::cards::get_card;
use crate::entity::Entity;
use crate::types::Phase;

pub fn process_effect_card_upgrade(id_target: usize, entities: &mut [Entity]) -> Option<Phase> {
    let name = entities[id_target].card_name;
    entities[id_target] = get_card(name, true);
    None
}
