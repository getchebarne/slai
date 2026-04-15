use crate::effect::{Effect, EffectKind, Target};
use crate::engine::ProcessEffectResult;

pub fn process_effect_calculated_gamble(hand: &[usize]) -> ProcessEffectResult {
    // Calculate number of cards to discard / draw
    let num_cards = hand.len();

    // For each card in the hand, create an effect to discard it
    let mut top: Vec<Effect> = hand
        .iter()
        .map(|&id_card| Effect {
            kind: EffectKind::CardDiscard,
            source: None,
            target: Target::Direct(Some(id_card)),
        })
        .collect();

    // Add a final effect to draw that many cards
    top.push(Effect {
        kind: EffectKind::CardDraw {
            count: num_cards as u8,
        },
        source: None,
        target: Target::Direct(None),
    });

    // Continue w/ top effects
    ProcessEffectResult::Continue {
        top,
        bot: Vec::new(),
    }
}
