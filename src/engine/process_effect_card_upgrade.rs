use crate::cards::get_card;
use crate::engine::DispatchResult;
use crate::entity::Entity;

pub fn process_effect_card_upgrade(id_target: usize, entities: &mut [Entity]) -> DispatchResult {
    let name = entities[id_target].card_name;
    entities[id_target] = get_card(name, true);
    DispatchResult::Continue
}
