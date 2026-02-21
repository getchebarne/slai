use crate::engine::ProcessEffectResult;
use crate::utils::remove_card_from_hand;

pub fn process_effect_card_discard(
    card_idx: usize,
    hand: &mut Vec<usize>,
    disc_pile: &mut Vec<usize>,
) -> ProcessEffectResult {
    remove_card_from_hand(card_idx, hand);
    disc_pile.push(card_idx);

    ProcessEffectResult::Pass
}
