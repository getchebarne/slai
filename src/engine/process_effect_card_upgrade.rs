use crate::cards::Card;
use crate::cards::get_card;
use crate::engine::ProcessEffectResult;

pub fn process_effect_card_upgrade(card_idx: usize, deck: &mut Vec<Card>) -> ProcessEffectResult {
    let card_old = deck[card_idx];
    deck[card_idx] = get_card(card_old.name, true);

    ProcessEffectResult::Pass
}
