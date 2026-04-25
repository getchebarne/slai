use crate::engine::DispatchResult;
use crate::utils::remove_card_from_hand;

pub fn process_effect_card_exhaust(
    id_card: usize,
    id_hand: &mut Vec<usize>,
    id_pile_exhaust: &mut Vec<usize>,
) -> DispatchResult {
    remove_card_from_hand(id_card, id_hand);
    id_pile_exhaust.push(id_card);

    // Continue
    DispatchResult::Continue
}
