use crate::effect::Effect;
use crate::engine::ProcessEffectResult;

pub fn process_effect_calculated_gamble(
    hand: &mut Vec<usize>,
) -> ProcessEffectResult {
    let num_cards = hand.len();

    ProcessEffectResult::Continue {
        bot: Vec::new(),
        top: vec![
            Effect::CardDiscardAll,
            Effect::CardDraw {
                count: num_cards as u8,
            },
        ],
    }
}
