use crate::cards::get_card;
use crate::consts::MAX_SIZE_HAND;
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::types::CardName;

pub fn process_effect_add_shivs(
    count: u8,
    entities: &mut Vec<Entity>,
    id_hand: &mut Vec<usize>,
    id_discard_pile: &mut Vec<usize>,
) -> DispatchResult {
    let shiv = get_card(CardName::Shiv, false);

    for _ in 0..count {
        let id_card = entities.len();
        entities.push(shiv);

        if id_hand.len() < MAX_SIZE_HAND {
            id_hand.push(id_card)
        } else {
            id_discard_pile.push(id_card)
        }
    }

    DispatchResult::Continue
}
