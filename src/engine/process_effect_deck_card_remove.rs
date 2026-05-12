use crate::engine::DispatchResult;
use crate::utils::remove_card_from_hand;

pub fn process_effect_deck_card_remove(id_card: usize, id_deck: &mut Vec<usize>) -> DispatchResult {
    remove_card_from_hand(id_card, id_deck);
    DispatchResult::Continue
}
