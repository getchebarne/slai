use crate::engine::ProcessEffectResult;
use crate::types::EntityId;
use crate::utils::remove_card_from_hand;

pub fn process_effect_card_discard(
    card_id: EntityId,
    hand: &mut Vec<EntityId>,
    disc_pile: &mut Vec<EntityId>,
) -> ProcessEffectResult {
    remove_card_from_hand(card_id, hand);
    disc_pile.push(card_id);
    ProcessEffectResult::Pass
}
