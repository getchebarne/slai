use crate::consts::MAX_SIZE_HAND;
use crate::engine::ProcessEffectResult;
use crate::types::EntityId;
use crate::utils::shuffle;

use rand::Rng;

pub fn process_effect_card_draw(
    count: u8,
    draw_pile: &mut Vec<EntityId>,
    hand: &mut Vec<EntityId>,
    discard_pile: &mut Vec<EntityId>,
    rng: &mut impl Rng,
) -> ProcessEffectResult {
    for _ in 0..count {
        if draw_pile.is_empty() {
            // Move discard pile cards to draw pile & shuffle draw pile
            // TODO: this should create a shuffle effect
            draw_pile.append(discard_pile);
            shuffle(draw_pile, rng);
        }

        // If the draw pile is still empty, no cards can be drawn. Should only
        // happen for very thin decks
        if draw_pile.is_empty() {
            break;
        }

        // Get the card's id and add it to the hand or discard pile according to hand length
        let id_card = draw_pile.pop().unwrap();
        if hand.len() < MAX_SIZE_HAND {
            hand.push(id_card);
        } else {
            discard_pile.push(id_card);
        }
    }

    // Continue
    ProcessEffectResult::Continue
}
