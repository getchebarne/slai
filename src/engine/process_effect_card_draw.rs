use crate::consts::MAX_SIZE_HAND;
use crate::engine::DispatchResult;
use crate::utils::shuffle;

use rand::Rng;

pub fn process_effect_card_draw(
    count: u8,
    id_pile_draw: &mut Vec<usize>,
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
    last_drawn_card: &mut Option<usize>,
    rng: &mut impl Rng,
) -> DispatchResult {
    for _ in 0..count {
        if id_pile_draw.is_empty() {
            // Move discard pile cards to draw pile & shuffle draw pile
            // TODO: this should create a shuffle effect
            id_pile_draw.append(id_pile_discard);
            shuffle(id_pile_draw, rng);
        }

        // If the draw pile is still empty, no cards can be drawn. Should only
        // happen for very thin decks
        if id_pile_draw.is_empty() {
            break;
        }

        // Get the card's id and add it to the hand or discard pile according to hand length
        let id_card = id_pile_draw.pop().unwrap();
        *last_drawn_card = Some(id_card);
        if id_hand.len() < MAX_SIZE_HAND {
            id_hand.push(id_card);
        } else {
            id_pile_discard.push(id_card);
        }
    }

    // Continue
    DispatchResult::Continue
}
