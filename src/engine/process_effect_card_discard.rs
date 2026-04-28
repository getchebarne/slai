use crate::engine::DispatchResult;
use crate::utils::remove_card_from_hand;

// Explicit discard: an effect (Acrobatics, Concentrate, CalculatedGamble,
// Unload, ToolsOfTheTrade, etc.) removes a card from the player's hand.
// Counts as a "discard this turn" for SneakyStrike / Eviscerate / etc.
//
// For the "move just-played card to discard pile" case, see
// `process_effect_card_move_to_discard` — same internal mutation, no counter
// increment, no discard-trigger fan-out
pub fn process_effect_card_discard(
    id_target: usize,
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
    this_turn_discards: &mut u8,
) -> DispatchResult {
    // Move card from hand to the discard pile
    remove_card_from_hand(id_target, id_hand);
    id_pile_discard.push(id_target);

    // Increment cards discarded this turn
    *this_turn_discards = this_turn_discards.saturating_add(1);

    // Continue
    DispatchResult::Continue
}
