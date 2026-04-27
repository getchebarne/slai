use std::collections::VecDeque;

use crate::consts::MAX_SIZE_HAND;
use crate::effect::Effect;
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::modifier::{ModifierKind, modifier_has};
use crate::utils::shuffle;

use rand::Rng;

// CardDraw: try to draw `count` cards from the draw pile (reshuffling discard
// in if empty). If the character has the `NoDraw` modifier (BulletTime),
// short-circuit immediately without drawing — matches StS
// `DrawCardAction.update()` early-return.
//
// Tracks the entity id of every card drawn this batch (regardless of whether
// it landed in hand or in discard due to hand cap), and after the loop
// pushes each drawn card's `card_on_draw_effects` to the queue. EndlessAgony
// uses this to spawn another copy. Drawn ids are pushed in reverse so the
// first-drawn card's on_draw fires first when the queue resumes.
pub fn process_effect_card_draw(
    count: u8,
    entities: &[Entity],
    id_character: usize,
    id_pile_draw: &mut Vec<usize>,
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
    last_drawn_card: &mut Option<usize>,
    rng: &mut impl Rng,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    if modifier_has(&entities[id_character].modifiers, ModifierKind::NoDraw) {
        return DispatchResult::Continue;
    }

    // Stack buf for drawn ids. CardDraw counts cap at hand size in practice,
    // but the call may request more than hand can hold (overflow lands in
    // discard). u8 count → at most 255 entries, but practical cap is ~20.
    let mut drawn = [0usize; 32];
    let mut drawn_n = 0;

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
        if drawn_n < drawn.len() {
            drawn[drawn_n] = id_card;
            drawn_n += 1;
        }
    }

    // Fire on-draw hooks for each drawn card (e.g., EndlessAgony's copy spawn).
    // Push in reverse so the first-drawn card's on_draw runs first when the
    // queue resumes.
    for &id_card in drawn[..drawn_n].iter().rev() {
        let on_draw = entities[id_card].card_on_draw_effects;
        for effect in on_draw.iter().rev() {
            queue.push_front(*effect);
        }
    }

    DispatchResult::Continue
}
