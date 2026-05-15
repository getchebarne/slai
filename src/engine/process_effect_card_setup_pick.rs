use crate::entity::Entity;
use crate::utils::remove_card_from_collection;
use crate::types::Phase;

pub fn process_effect_card_setup_pick(
    id_target: usize,
    entities: &mut [Entity],
    id_hand: &mut Vec<usize>,
    id_pile_draw: &mut Vec<usize>,
) -> Option<Phase> {
    // Set free-to-play-once flag
    entities[id_target].card_free_to_play_once = true;

    // Push to the front of the draw pile
    remove_card_from_collection(id_target, id_hand);
    id_pile_draw.push(id_target);

    // Continue
    None
}
