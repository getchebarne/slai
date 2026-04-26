use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::utils::remove_card_from_hand;

// Setup direct-form: the player picked a card. Set its
// `card_free_to_play_once` flag and move it from hand to the top of the
// draw pile (the end of `id_pile_draw` since draw pops from the back).
pub fn process_effect_card_setup_pick(
    id_card: usize,
    entities: &mut [Entity],
    id_hand: &mut Vec<usize>,
    id_pile_draw: &mut Vec<usize>,
) -> DispatchResult {
    entities[id_card].card_free_to_play_once = true;
    remove_card_from_hand(id_card, id_hand);
    id_pile_draw.push(id_card);
    DispatchResult::Continue
}
