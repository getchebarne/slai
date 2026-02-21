use crate::effect::Effect;
use crate::engine::ProcessEffectResult;
use crate::utils::remove_card_from_hand;

pub fn process_effect_calculated_gamble(
    card_idx: usize,
    hand: &mut Vec<usize>,
    disc_pile: &mut Vec<usize>,
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
