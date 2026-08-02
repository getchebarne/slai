use rand::Rng;

use crate::consts::MAX_SIZE_HAND;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::types::CostScope;
use crate::types::Mode;
use crate::types::RelicName;
use crate::utils::has_relic;

// NoDraw short-circuits. on_draw hooks fire after the full batch, in draw order
pub fn process_effect_card_draw(state: &mut GameState, count: u16) {
    let Some(Mode::Combat {
        id_hand,
        id_pile_draw,
        id_pile_discard,
        id_card_last_drawn,
        ..
    }) = state.mode_stack.last_mut()
    else {
        unreachable!("process_effect_card_draw outside Combat mode")
    };
    if has_modifier(
        &state.entities[state.id_character].modifiers,
        ModifierKind::NoDraw,
    ) {
        return;
    }

    // Initialize variables to track IDs and count of drawn cards, and wether reshuffle is needed
    let mut id_drawn = [0usize; 32];
    let mut id_drawn_num = 0;
    let mut shuffle_resume_remaining: Option<u16> = None;

    // Try to draw all cards
    for i in 0..count {
        if id_pile_draw.is_empty() {
            if id_pile_discard.is_empty() {
                // Nothing to draw from
                break;
            }

            // Need to reshuffle and re-draw the remaining count
            shuffle_resume_remaining = Some(count - i);
            break;
        }

        // Remove card from draw pile
        let id_card = id_pile_draw.pop().unwrap();

        // Place it in the hand if there's room, discard pile otherwise
        if id_hand.len() < MAX_SIZE_HAND {
            id_hand.push(id_card);
            *id_card_last_drawn = Some(id_card);
        } else {
            id_pile_discard.push(id_card);
        }

        // Update drawn IDs and count
        if id_drawn_num < id_drawn.len() {
            id_drawn[id_drawn_num] = id_card;
            id_drawn_num += 1;
        }
    }

    // Reshuffle -> redraw if needed
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

    // Snecko Eye: every drawn card's cost re-rolls to [0, 3]
    if has_relic(&state.id_relics, RelicName::SneckoEye) {
        for &id_card in &id_drawn[..id_drawn_num] {
            let card = &state.entities[id_card];

            // XCost and unplayable skip the roll
            if matches!(card.card_cost_kind, CardCostKind::XCost { .. })
                || card.card_play_restriction == PlayRestriction::Never
            {
                continue;
            }

            // Roll new cost
            let new_cost: u8 = state.rng.random_range(0..=3);

            // Only push it if it's different from the original
            if new_cost != card.card_cost {
                state.effect_queue.push_front(Effect {
                    kind: EffectKind::SetCostOverride {
                        amount: new_cost,
                        only_reduce: false,
                        random: false,
                        scope: CostScope::Combat, // Combat-scope ensures redraws re-roll
                    },
                    id_source: None,
                    target: Target::Direct(Some(id_card)),
                });
            }
        }
    }

    // Fire on-draw hooks in draw order; push reversed so front-of-queue resumes correctly
    for &id_card in id_drawn[..id_drawn_num].iter().rev() {
        let effects_on_draw = state.entities[id_card].card_on_draw_effects;
        for effect in effects_on_draw.iter().rev() {
            state.effect_queue.push_front(Effect {
                id_source: Some(id_card),
                ..*effect
            });
        }
    }
}
