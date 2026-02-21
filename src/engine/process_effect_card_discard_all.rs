use crate::engine::ProcessEffectResult;

pub fn process_effect_card_discard_all(
    hand: &mut Vec<usize>,
    disc_pile: &mut Vec<usize>,
) -> ProcessEffectResult {
    let hand_copy: Vec<usize> = hand.drain(..).collect();
    disc_pile.extend(hand_copy);

    ProcessEffectResult::Pass
}
