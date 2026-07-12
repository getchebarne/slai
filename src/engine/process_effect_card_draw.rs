use crate::consts::MAX_SIZE_HAND;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;

// NoDraw short-circuits. on_draw hooks fire after the full batch, in draw order
pub fn process_effect_card_draw(state: &mut GameState, count: u16) {
    if modifier_has(
        &state.entities[state.id_character].modifiers,
        ModifierKind::NoDraw,
    ) {
        return;
    }

    let mut id_drawn = [0usize; 32];
    let mut id_drawn_n = 0;
    let mut shuffle_resume_remaining: Option<u16> = None;

    for i in 0..count {
        if state.id_pile_draw.is_empty() {
            if state.id_pile_discard.is_empty() {
                break;
            }
            shuffle_resume_remaining = Some(count - i);
            break;
        }

        let id_card = state.id_pile_draw.pop().unwrap();

        if state.id_hand.len() < MAX_SIZE_HAND {
            state.id_hand.push(id_card);
            state.id_card_last_drawn = Some(id_card);
        } else {
            state.id_pile_discard.push(id_card);
        }
        if id_drawn_n < id_drawn.len() {
            id_drawn[id_drawn_n] = id_card;
            id_drawn_n += 1;
        }
    }

    if let Some(remaining) = shuffle_resume_remaining {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardDraw { count: remaining },
            id_source: None,
            target: Target::Direct(None),
        });
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ShuffleDiscardPileIntoDrawPile,
            id_source: None,
            target: Target::Direct(None),
        });
    }

    // Fire on-draw hooks in draw order; push reversed so front-of-queue resumes correctly
    for &id_card in id_drawn[..id_drawn_n].iter().rev() {
        let effects_on_draw = state.entities[id_card].card_on_draw_effects;
        for effect in effects_on_draw.iter().rev() {
            state.effect_queue.push_front(Effect {
                id_source: Some(id_card),
                ..*effect
            });
        }
    }
}
