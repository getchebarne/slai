use crate::types::Phase;
use crate::utils::remove_card_from_collection;

pub fn process_effect_card_remove(id_card: usize, id_hand: &mut Vec<usize>) -> Option<Phase> {
    // Remove card from the hand
    remove_card_from_collection(id_card, id_hand);

    // Continue
    None
}
