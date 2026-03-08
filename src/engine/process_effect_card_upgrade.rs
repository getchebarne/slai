use crate::cards::get_card;
use crate::engine::ProcessEffectResult;
use crate::state::Entity;
use crate::types::EntityId;

pub fn process_effect_card_upgrade(
    id_card: usize,
    deck: &[EntityId],
    entities: &mut Vec<Entity>,
) -> ProcessEffectResult {
    // Replace card with its upgraded version
    let entity_id = deck[id_card];
    let card = entities[entity_id.0 as usize].kind.card_mut();
    *card = get_card(card.name, true);

    ProcessEffectResult::Continue
}
