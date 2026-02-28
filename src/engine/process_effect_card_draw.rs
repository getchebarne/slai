use crate::consts::MAX_SIZE_HAND;
use crate::engine::ProcessEffectResult;
use crate::types::EntityId;
use crate::utils::shuffle;

use rand::Rng;

pub fn process_effect_card_draw(
    count: u8,
    draw_pile: &mut Vec<EntityId>,
    hand: &mut Vec<EntityId>,
    disc_pile: &mut Vec<EntityId>,
    rng: &mut impl Rng,
) -> ProcessEffectResult {
    for _ in 0..count {
        if draw_pile.is_empty() {
            // TODO: this should create a shuffle effect
            draw_pile.append(disc_pile);
            shuffle(draw_pile, rng);
        }

        if draw_pile.is_empty() {
            break;
        }

        let card_id = draw_pile.pop().unwrap();
        if hand.len() < MAX_SIZE_HAND {
            hand.push(card_id);
        } else {
            disc_pile.push(card_id);
        }
    }
    ProcessEffectResult::Pass
}
