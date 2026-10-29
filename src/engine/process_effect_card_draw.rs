use rand::Rng;

use crate::consts::MAX_SIZE_CARD_PILE_HAND;
use crate::effect::EFFECT_ACCURACY_RESYNC_HAND;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::types::Combat;
use crate::types::CostScope;

pub fn process_effect_card_draw(state: &mut GameState, count: u16) {
    assert!(
        state.combat.active,
        "process_effect_card_draw outside the Combat frame"
    );
    let Combat {
        id_card_pile_hand,
        id_card_pile_draw,
        id_card_pile_discard,
        id_card_last_drawn,
        ..
    } = &mut state.combat;

    // Clear the draw history before every bail path
    *id_card_last_drawn = None;
    if has_modifier(
        &state.entities[state.id_character].modifiers,
        ModifierKind::NoDraw,
    ) {
        return;
    }

    // Both piles empty: nothing to draw and no reshuffle
    if id_card_pile_draw.is_empty() && id_card_pile_discard.is_empty() {
        return;
    }

    // Overdraw never happens: the excess stays on the draw pile
    let count = count.min(MAX_SIZE_CARD_PILE_HAND.saturating_sub(id_card_pile_hand.len()) as u16);

    // Out of cards partway: the reshuffle's Relic hooks fire before any Card is drawn, and even an empty discard pile reshuffles
    let draw_pile_size = id_card_pile_draw.len() as u16;
    if count > draw_pile_size {
        let remaining = count - draw_pile_size;

        // Executes in reverse:
        //     1. ShuffleRelicHooks
        //     2. CardDraw (the draw pile)
        //     3. ShuffleCardPileDiscardIntoCardPileDraw
        //     4. CardDraw (remaining)
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardDraw { count: remaining },
            id_source: None,
            target: Target::Direct(None),
        });
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ShuffleCardPileDiscardIntoCardPileDraw,
            id_source: None,
            target: Target::Direct(None),
        });
        if draw_pile_size > 0 {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardDraw {
                    count: draw_pile_size,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ShuffleRelicHooks,
            id_source: None,
            target: Target::Direct(None),
        });
        return;
    }

    // Initialize variables to track IDs and count of drawn Cards
    let mut id_drawn = [0usize; 32];
    let mut id_drawn_num = 0;

    // Draw all Cards
    for _ in 0..count {
        // Remove Card from draw pile
        let id_card = id_card_pile_draw.pop().unwrap();
        id_card_pile_hand.push(id_card);
        *id_card_last_drawn = Some(id_card);

        // Update drawn IDs and count
        if id_drawn_num < id_drawn.len() {
            id_drawn[id_drawn_num] = id_card;
            id_drawn_num += 1;
        }
    }

    // Confusion (Snecko Eye, Snecko's Glare): every drawn Card's cost re-rolls to [0, 3]
    if has_modifier(
        &state.entities[state.id_character].modifiers,
        ModifierKind::Confusion,
    ) {
        for &id_card in &id_drawn[..id_drawn_num] {
            let card = &state.entities[id_card];

            // XCost and unplayable skip the roll
            if matches!(card.card_cost_kind, CardCostKind::XCost { .. })
                || card.card_play_restriction == PlayRestriction::Never
            {
                continue;
            }
            let card_cost = card.card_cost;

            // Roll new cost
            let new_cost: u16 = state.rng.random_range(0..=3);

            // Free-to-play-once is spent whether or not the roll changed anything, once the on-draw hooks have seen it
            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardFreePlaySpend,
                id_source: None,
                target: Target::Direct(Some(id_card)),
            });

            // Only push it if it's different from the original
            if new_cost != card_cost {
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

    // A drawn Eviscerate restarts its cost this turn, ahead of the re-roll
    for &id_card in &id_drawn[..id_drawn_num] {
        if state.entities[id_card].card_cost_kind == CardCostKind::MinusDiscardsThisTurn {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardCostMinusDiscards,
                id_source: None,
                target: Target::Direct(Some(id_card)),
            });
        }
    }

    // On-draw hooks see each Card as drawn: ahead of its re-roll and any reshuffle, the last-drawn Card's first; an energy change (Void's loss) lands behind everything queued instead
    for &id_card in &id_drawn[..id_drawn_num] {
        let effects_draw = state.entities[id_card].card_effects_draw;
        for effect in effects_draw.iter().rev() {
            let effect = Effect {
                id_source: Some(id_card),
                ..*effect
            };
            if matches!(effect.kind, EffectKind::EnergyDelta { .. }) {
                state.effect_queue.push_back(effect);
            } else {
                state.effect_queue.push_front(effect);
            }
        }
    }

    // Accuracy: drawing resets every Shiv in the hand
    if id_drawn_num > 0
        && has_modifier(
            &state.entities[state.id_character].modifiers,
            ModifierKind::Accuracy,
        )
    {
        state.effect_queue.push_front(EFFECT_ACCURACY_RESYNC_HAND);
    }
}
