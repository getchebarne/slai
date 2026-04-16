use crate::engine::DispatchResult;
use crate::utils::remove_card_from_hand;

pub fn process_effect_card_discard(
    id_card: usize,
    hand: &mut Vec<usize>,
    discard_pile: &mut Vec<usize>,
) -> DispatchResult {
    // Remove card from hand and send it to the discard pile
    remove_card_from_hand(id_card, hand);
    discard_pile.push(id_card);

    // Continue
    DispatchResult::Continue
}
