use crate::engine::DispatchResult;
use crate::engine::process_effect_card_exhaust::process_effect_card_exhaust;
use crate::engine::process_effect_card_move_to_discard::process_effect_card_move_to_discard;
use crate::entity::Entity;

pub fn process_effect_card_discard_end_of_turn(
    id_target: usize,
    entities: &mut [Entity],
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
    id_pile_exhaust: &mut Vec<usize>,
) -> DispatchResult {
    if entities[id_target].card_retain {
        entities[id_target].card_retain = false;
        return DispatchResult::Continue;
    }
    if entities[id_target].card_ethereal {
        // Ethereal: auto-exhaust at end of turn instead of discard.
        return process_effect_card_exhaust(id_target, id_hand, id_pile_exhaust);
    }
    // End-of-turn discard does not count as an explicit discard (no
    // `this_turn_discards`` increment, no Reflex/Tactician trigger, etc.)
    process_effect_card_move_to_discard(id_target, id_hand, id_pile_discard)
}
