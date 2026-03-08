use crate::engine::ProcessEffectResult;
use crate::types::EntityId;
use crate::utils::remove_card_from_hand;

pub fn process_effect_card_remove(
    id_card: EntityId,
    hand: &mut Vec<EntityId>,
) -> ProcessEffectResult {
    // Remove card from the hand
    remove_card_from_hand(id_card, hand);

    // Continue
    ProcessEffectResult::Continue
}
