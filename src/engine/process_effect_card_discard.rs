use crate::engine::ProcessEffectResult;
use crate::types::EntityId;
use crate::utils::remove_card_from_hand;

pub fn process_effect_card_discard(
    id_card: EntityId,
    hand: &mut Vec<EntityId>,
    disc_pile: &mut Vec<EntityId>,
) -> ProcessEffectResult {
    // Remove card from hand and send it to the discard pile
    remove_card_from_hand(id_card, hand);
    disc_pile.push(id_card);

    // Continue
    ProcessEffectResult::Continue
}
