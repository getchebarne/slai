use crate::consts::MAX_SIZE_HAND;
use crate::engine::ProcessEffectResult;

use rand::Rng;

pub fn process_effect_card_draw(
    count: u8,
    draw_pile: &mut Vec<usize>,
    hand: &mut Vec<usize>,
    disc_pile: &mut Vec<usize>,
    rng: &mut impl Rng,
) -> ProcessEffectResult {
    for _ in 0..count {
        if draw_pile.is_empty() {
            // Shuffle discard pile into draw pile
            // TODO: this should create a shuffle effect
            draw_pile.append(disc_pile);

            // Fisher-Yates shuffle
            let draw_pile_len = draw_pile.len();
            for i in (1..draw_pile_len).rev() {
                let j = rng.random_range(0..=i);
                draw_pile.swap(i, j);
            }
        }

        if draw_pile.is_empty() {
            // If the draw pile is empty after shuffling the discard pile into it, break
            // Can only happen for very small decks (<= 10 cards)
            break;
        }

        // Draw one card
        let card_idx = draw_pile.pop().unwrap();
        if hand.len() < MAX_SIZE_HAND {
            hand.push(card_idx);
        } else {
            disc_pile.push(card_idx);
        }
    }
    ProcessEffectResult::Pass
}
