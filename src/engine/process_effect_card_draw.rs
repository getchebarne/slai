use std::collections::VecDeque;

use crate::consts::MAX_SIZE_HAND;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::entity::Entity;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;


// NoDraw short-circuits. on_draw hooks fire after the full batch, in draw order
pub fn process_effect_card_draw(
    count: u16,
    entities: &[Entity],
    id_character: usize,
    id_pile_draw: &mut Vec<usize>,
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
    card_last_drawn: &mut Option<usize>,
    effect_queue: &mut VecDeque<Effect>,
) {
    if modifier_has(&entities[id_character].modifiers, ModifierKind::NoDraw) {
        return;
    }

    let mut id_drawn = [0usize; 32];
    let mut id_drawn_n = 0;
    let mut shuffle_resume_remaining: Option<u16> = None;

    for i in 0..count {
        if id_pile_draw.is_empty() {
            // Thin deck: nothing in either pile, give up
            if id_pile_discard.is_empty() {
                break;
            }
            // Defer the shuffle + remaining draws to the queue so the shuffle
            // is observable and any halts ride between the two events
            shuffle_resume_remaining = Some(count - i);
            break;
        }

        let id_card = id_pile_draw.pop().unwrap();

        if id_hand.len() < MAX_SIZE_HAND {
            id_hand.push(id_card);
            // Escape Plan reads card_last_drawn to decide if it grants block
            *card_last_drawn = Some(id_card);
        } else {
            id_pile_discard.push(id_card);
        }
        if id_drawn_n < id_drawn.len() {
            id_drawn[id_drawn_n] = id_card;
            id_drawn_n += 1;
        }
    }

    // Push resume effects FIRST so on_draw hooks (pushed below) end up at the
    // front of the queue and fire before the shuffle
    if let Some(remaining) = shuffle_resume_remaining {
        effect_queue.push_front(Effect::direct(
            EffectKind::CardDraw { count: remaining },
            None,
            None,
        ));
        effect_queue.push_front(Effect::direct(
            EffectKind::ShuffleDiscardPileIntoDrawPile,
            None,
            None,
        ));
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
}
