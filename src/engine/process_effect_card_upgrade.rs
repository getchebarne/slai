use crate::cards::get_card;
use crate::engine::ProcessEffectResult;
use crate::state::Entity;
use crate::types::EntityId;

pub fn process_effect_card_upgrade(
    card_idx: usize,
    deck: &[EntityId],
    entities: &mut Vec<Entity>,
) -> ProcessEffectResult {
    let entity_id = deck[card_idx];
    let card = entities[entity_id.0 as usize].kind.card_mut();
    *card = get_card(card.name, true);

    ProcessEffectResult::Pass
}
