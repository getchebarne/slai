use crate::cards::Card;
use crate::cards::get_card;
use crate::consts::MAX_SIZE_HAND;
use crate::engine::ProcessEffectResult;
use crate::types::CardName;
use crate::utils::remove_card_from_hand;

pub fn process_effect_add_shivs(
    count: u8,
    combat_cards: &mut Vec<Card>,
    hand: &mut Vec<usize>,
    disc_pile: &mut Vec<usize>,
) -> ProcessEffectResult {
    // Create the Shivs
    let shiv = get_card(CardName::Shiv, false);

    for _ in 0..count {
        let card_idx = combat_cards.len();
        combat_cards.push(shiv);

        if hand.len() < MAX_SIZE_HAND {
            hand.push(card_idx)
        } else {
            disc_pile.push(card_idx)
        }
    }

    ProcessEffectResult::Pass
}
