use crate::effect::{Effect, EffectKind};
use crate::engine::ProcessEffectResult;
use crate::types::EntityId;

pub fn process_effect_calculated_gamble(hand: &[EntityId]) -> ProcessEffectResult {
    let num_cards = hand.len();

    let mut top: Vec<Effect> = hand
        .iter()
        .map(|&card_id| Effect {
            kind: EffectKind::CardDiscard,
            source: None,
            target: Some(card_id),
        })
        .collect();

    top.push(Effect {
        kind: EffectKind::CardDraw {
            count: num_cards as u8,
        },
        source: None,
        target: None,
    });

    ProcessEffectResult::AddAndContinue {
        top,
        bot: Vec::new(),
    }
}
