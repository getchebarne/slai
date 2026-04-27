use crate::cards::get_card;
use crate::engine::DispatchResult;
use crate::entity::{Entity, add_card_to_hand_or_discard};
use crate::types::CardName;

// EndlessAgony's `triggerWhenDrawn` hook spawn-handler. Creates a fresh
// EndlessAgony (or EndlessAgony+ if `upgraded`) and routes it to hand or
// discard via the shared spawn helper.
pub fn process_effect_endless_agony_add_copy(
    upgraded: bool,
    entities: &mut Vec<Entity>,
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
) -> DispatchResult {
    let card = get_card(CardName::EndlessAgony, upgraded);
    add_card_to_hand_or_discard(entities, id_hand, id_pile_discard, card);
    DispatchResult::Continue
}
