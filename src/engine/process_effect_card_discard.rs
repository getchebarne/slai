use std::collections::VecDeque;

use crate::effect::Effect;
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::utils::remove_card_from_hand;

// Explicit discard (Acrobatics, Concentrate, CalculatedGamble, Unload, ToolsOfTheTrade, etc.).
// For the "move just-played card to discard pile" case, see process_effect_card_move_to_discard
pub fn process_effect_card_discard(
    id_target: usize,
    entities: &[Entity],
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
    cards_discarded_this_turn: &mut u8,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    remove_card_from_hand(id_target, id_hand);
    id_pile_discard.push(id_target);
    *cards_discarded_this_turn = cards_discarded_this_turn.saturating_add(1);

    // On-discard effects. Push in reverse so the first effect runs first when the queue resumes.
    let effects_on_discard = entities[id_target].card_on_discard_effects;
    for effect in effects_on_discard.iter().rev() {
        queue.push_front(*effect);
    }
    DispatchResult::Continue
}
