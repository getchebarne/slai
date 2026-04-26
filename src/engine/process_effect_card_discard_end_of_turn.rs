use crate::engine::DispatchResult;
use crate::engine::process_effect_card_discard::process_effect_card_discard;
use crate::entity::Entity;

pub fn process_effect_card_discard_end_of_turn(
    id_card: usize,
    entities: &mut [Entity],
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
) -> DispatchResult {
    if entities[id_card].card_retain {
        entities[id_card].card_retain = false;
        return DispatchResult::Continue;
    }
    process_effect_card_discard(id_card, id_hand, id_pile_discard)
}
