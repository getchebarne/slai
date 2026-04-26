use std::collections::VecDeque;

use crate::effect::Effect;
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::utils::remove_card_from_hand;

// Explicit discard: an effect (Acrobatics, Concentrate, CalculatedGamble,
// Unload, ToolsOfTheTrade, etc.) removes a card from the player's hand.
// Counts as a "discard this turn" for SneakyStrike / Eviscerate / etc., and
// fires Reflex/Tactician's `card_on_discard_effects`.
//
// For the "move just-played card to discard pile" case, see
// `process_effect_card_move_to_discard` — same internal mutation, no counter
// increment, no on-discard trigger.
pub fn process_effect_card_discard(
    id_card: usize,
    entities: &[Entity],
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
    discards_this_turn: &mut u8,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    remove_card_from_hand(id_card, id_hand);
    id_pile_discard.push(id_card);
    *discards_this_turn = discards_this_turn.saturating_add(1);

    // Fire on-discard trigger (Reflex, Tactician). Push in reverse so the
    // first effect in the array runs first when the queue resumes.
    let on_discard = entities[id_card].card_on_discard_effects;
    for effect in on_discard.iter().rev() {
        queue.push_front(*effect);
    }
    DispatchResult::Continue
}
