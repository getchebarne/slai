use crate::engine::DispatchResult;
use crate::utils::remove_card_from_hand;

pub fn process_effect_card_remove(id_card: usize, id_hand: &mut Vec<usize>) -> DispatchResult {
    // Remove card from the hand
    remove_card_from_hand(id_card, id_hand);

    // Continue
    DispatchResult::Continue
}
