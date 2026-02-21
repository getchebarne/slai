use crate::engine::ProcessEffectResult;
use crate::utils::remove_card_from_hand;

pub fn process_effect_card_exhaust(
    card_idx: usize,
    hand: &mut Vec<usize>,
    exh_pile: &mut Vec<usize>,
) -> ProcessEffectResult {
    remove_card_from_hand(card_idx, hand);
    exh_pile.push(card_idx);

    ProcessEffectResult::Pass
}
