use crate::engine::ProcessEffectResult;
use crate::types::EntityId;
use crate::utils::remove_card_from_hand;

pub fn process_effect_card_exhaust(
    card_id: EntityId,
    hand: &mut Vec<EntityId>,
    exh_pile: &mut Vec<EntityId>,
) -> ProcessEffectResult {
    remove_card_from_hand(card_id, hand);
    exh_pile.push(card_id);
    ProcessEffectResult::Pass
}
