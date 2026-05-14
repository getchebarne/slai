use std::collections::VecDeque;

use crate::consts::MAX_SIZE_HAND;
use crate::effect::Effect;
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::utils::shuffle;

use rand::Rng;

// NoDraw short-circuits. on_draw hooks fire after the full batch, in draw order
pub fn process_effect_card_draw(
    count: u8,
    entities: &[Entity],
    id_character: usize,
    id_pile_draw: &mut Vec<usize>,
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
    card_last_drawn: &mut Option<usize>,
    rng: &mut impl Rng,
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    if modifier_has(&entities[id_character].modifiers, ModifierKind::NoDraw) {
        // Early return
        return DispatchResult::Continue;
    }

    // Buffer to track drawn card IDs
    let mut id_drawn = [0usize; 32];
    let mut id_drawn_n = 0;

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

        if id_hand.len() < MAX_SIZE_HAND {
            id_hand.push(id_card);

            // Set last drawn card. Escape plan uses this to decide if it needs to increment block
            *card_last_drawn = Some(id_card);
        } else {
            id_pile_discard.push(id_card);
        }
        if id_drawn_n < id_drawn.len() {
            id_drawn[id_drawn_n] = id_card;
            id_drawn_n += 1;
        }
    }

    // Fire on-draw hooks for each drawn card (e.g., EndlessAgony's copy spawn)
    // Push in reverse so the first-drawn card's on_draw runs first when the
    // effect_queue resumes
    // TODO: these should trigger immediately, may have to remove `count` from here
    // and push individual single-card draw effects
    for &id_card in id_drawn[..id_drawn_n].iter().rev() {
        let effects_on_draw = entities[id_card].card_on_draw_effects;
        for effect in effects_on_draw.iter().rev() {
            effect_queue.push_front(Effect {
                id_source: Some(id_card),
                ..*effect
            });
        }
    }

    DispatchResult::Continue
}
