use crate::cards::get_card;
use crate::engine::ProcessEffectResult;
use crate::state::{Entity, EntityKind};
use crate::types::EntityId;

pub fn process_effect_card_upgrade(
    target: EntityId,
    entities: &mut [Entity],
) -> ProcessEffectResult {
    // Replace card with its upgraded version
    let EntityKind::Card(card) = &mut entities[target.0 as usize].kind else { unreachable!() };
    *card = get_card(card.name, true);

    ProcessEffectResult::Continue
}
