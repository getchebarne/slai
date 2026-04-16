use crate::cards::get_card;
use crate::engine::DispatchResult;
use crate::entity::Entity;

pub fn process_effect_card_upgrade(
    target: usize,
    entities: &mut [Entity],
) -> DispatchResult {
    let name = entities[target].card_name;
    entities[target] = get_card(name, true);
    DispatchResult::Continue
}
