use crate::cards::get_card;
use crate::engine::DispatchResult;
use crate::entity::{Entity, add_card_to_hand_or_discard};
use crate::types::CardName;

pub fn process_effect_card_add_to_hand(
    card_name: CardName,
    count: u8,
    upgraded: bool,
    entities: &mut Vec<Entity>,
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
) -> DispatchResult {
    if count == 0 {
        return DispatchResult::Continue;
    }
    for _ in 0..count {
        let card = get_card(card_name, upgraded);
        add_card_to_hand_or_discard(entities, id_hand, id_pile_discard, card);
    }
    DispatchResult::Continue
}
