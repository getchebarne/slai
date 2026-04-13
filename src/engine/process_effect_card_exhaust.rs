use crate::engine::ProcessEffectResult;
use crate::utils::remove_card_from_hand;

pub fn process_effect_card_exhaust(
    id_card: usize,
    hand: &mut Vec<usize>,
    exh_pile: &mut Vec<usize>,
) -> ProcessEffectResult {
    // Remove card from hand and send it to the exhaust pile
    remove_card_from_hand(id_card, hand);
    exh_pile.push(id_card);

    // Continue
    ProcessEffectResult::Continue
}
