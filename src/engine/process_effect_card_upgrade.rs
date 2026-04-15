use crate::cards::get_card;
use crate::engine::ProcessEffectResult;
use crate::entity::Entity;

pub fn process_effect_card_upgrade(
    target: usize,
    entities: &mut [Entity],
) -> ProcessEffectResult {
    let name = entities[target].card_name;
    entities[target] = get_card(name, true);
    ProcessEffectResult::Continue { top: vec![], bot: vec![] }
}
