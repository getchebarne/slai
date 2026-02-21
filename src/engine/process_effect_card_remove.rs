use crate::engine::ProcessEffectResult;
use crate::utils::remove_card_from_hand;

pub fn process_effect_card_remove(card_idx: usize, hand: &mut Vec<usize>) -> ProcessEffectResult {
    remove_card_from_hand(card_idx, hand);

    ProcessEffectResult::Pass
}
