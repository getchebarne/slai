use crate::types::Phase;
use crate::utils::remove_card_from_collection;

pub fn process_effect_card_remove_from_deck(
    id_card: usize,
    id_deck: &mut Vec<usize>,
) -> Option<Phase> {
    remove_card_from_collection(id_card, id_deck);
    None
}
