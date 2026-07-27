use crate::consts::MAX_SIZE_HAND;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::types::Mode;

// NoDraw short-circuits. on_draw hooks fire after the full batch, in draw order
pub fn process_effect_card_draw(state: &mut GameState, count: u16) {
    let Mode::Combat {
        id_hand,
        id_pile_draw,
        id_pile_discard,
        id_card_last_drawn,
        ..
    } = &mut state.mode
    else {
        unreachable!("process_effect_card_draw outside Combat mode")
    };
    if has_modifier(
        &state.entities[state.id_character].modifiers,
        ModifierKind::NoDraw,
    ) {
        return;
    }

    let mut id_drawn = [0usize; 32];
    let mut id_drawn_n = 0;
    let mut shuffle_resume_remaining: Option<u16> = None;

    for i in 0..count {
        if id_pile_draw.is_empty() {
            if id_pile_discard.is_empty() {
                break;
            }
            shuffle_resume_remaining = Some(count - i);
            break;
        }

        let id_card = id_pile_draw.pop().unwrap();

        if id_hand.len() < MAX_SIZE_HAND {
            id_hand.push(id_card);
            *id_card_last_drawn = Some(id_card);
        } else {
            id_pile_discard.push(id_card);
        }
        if id_drawn_n < id_drawn.len() {
            id_drawn[id_drawn_n] = id_card;
            id_drawn_n += 1;
        }
    }

    if let Some(remaining) = shuffle_resume_remaining {
        // Executes in reverse:
        //     1. ShuffleDiscardPileIntoDrawPile
        //     2. CardDraw (remaining)
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
