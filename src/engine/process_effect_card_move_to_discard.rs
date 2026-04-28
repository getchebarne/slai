use crate::engine::DispatchResult;
use crate::utils::remove_card_from_hand;

// Move-after-play: pushed by `process_effect_card_play` to send the
// just-played card to the discard pile. Same hand/discard mutation as
// `process_effect_card_discard`, but does NOT count as a "discard this turn"
// and does NOT fire Reflex/Tactician triggers — playing a card is not the
// same as discarding it.
pub fn process_effect_card_move_to_discard(
    id_target: usize,
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
) -> DispatchResult {
    remove_card_from_hand(id_target, id_hand);
    id_pile_discard.push(id_target);
    DispatchResult::Continue
}
