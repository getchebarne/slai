use crate::engine::DispatchResult;
use crate::utils::remove_card_from_hand;

pub fn process_effect_card_discard(
    id_target: usize,
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
) -> DispatchResult {
    // Remove card from hand and send it to the discard pile
    remove_card_from_hand(id_target, id_hand);
    id_pile_discard.push(id_target);

    // Continue
    DispatchResult::Continue
}
