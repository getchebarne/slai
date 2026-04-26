use crate::engine::DispatchResult;
use crate::utils::remove_card_from_hand;

// Explicit discard: an effect (Acrobatics, Concentrate, CalculatedGamble,
// Unload, ToolsOfTheTrade, etc.) removes a card from the player's hand.
// Counts as a "discard this turn" for SneakyStrike / Eviscerate / etc., and
// in the future will fire Reflex/Tactician's triggerOnManualDiscard.
//
// For the "move just-played card to discard pile" case, see
// `process_effect_card_move_to_discard` — same internal mutation, no counter
// increment, no discard-trigger fan-out.
pub fn process_effect_card_discard(
    id_card: usize,
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
    discards_this_turn: &mut u8,
) -> DispatchResult {
    remove_card_from_hand(id_card, id_hand);
    id_pile_discard.push(id_card);
    *discards_this_turn = discards_this_turn.saturating_add(1);
    DispatchResult::Continue
}
