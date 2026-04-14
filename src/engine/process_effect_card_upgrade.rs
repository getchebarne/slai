use crate::cards::get_card;
use crate::engine::ProcessEffectResult;
use crate::entities::{Entity, EntityKind};

pub fn process_effect_card_upgrade(
    target: usize,
    entities: &mut [Entity],
) -> ProcessEffectResult {
    // Replace card with its upgraded version
    let EntityKind::Card(card) = &mut entities[target].kind else { unreachable!() };
    *card = get_card(card.name, true);

    ProcessEffectResult::Continue
}
