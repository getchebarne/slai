use crate::effect::Effect;
use crate::engine::ProcessEffectResult;
use crate::types::EntityId;

pub fn process_effect_calculated_gamble(
    hand: &mut [EntityId],
) -> ProcessEffectResult {
    let num_cards = hand.len();

    ProcessEffectResult::Continue {
        bot: Vec::new(),
        top: vec![
            Effect::CardDiscardAll,
            Effect::CardDraw { count: num_cards as u8 },
        ],
    }
}
