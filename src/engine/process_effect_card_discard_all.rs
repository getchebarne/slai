use crate::engine::ProcessEffectResult;
use crate::types::EntityId;

pub fn process_effect_card_discard_all(
    hand: &mut Vec<EntityId>,
    disc_pile: &mut Vec<EntityId>,
) -> ProcessEffectResult {
    disc_pile.append(hand);
    ProcessEffectResult::Pass
}
