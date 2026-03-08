use crate::effect::{Effect, EffectKind};
use crate::engine::ProcessEffectResult;
use crate::types::EntityId;

pub fn process_effect_calculated_gamble(hand: &mut [EntityId]) -> ProcessEffectResult {
    // Calculate number of cards to draw
    let num_cards = hand.len();

    // Continue w/ top effects to discard all cards in the hand and
    // another one to draw that many
    ProcessEffectResult::AddAndContinue {
        bot: Vec::new(),
        top: vec![
            Effect {
                kind: EffectKind::CardDiscardAll,
                source: None,
                target: None,
            },
            Effect {
                kind: EffectKind::CardDraw {
                    count: num_cards as u8,
                },
                source: None,
                target: None,
            },
        ],
    }
}
