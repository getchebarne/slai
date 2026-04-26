use crate::cards::get_card;
use crate::consts::MAX_SIZE_HAND;
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::types::CardName;

pub fn process_effect_shiv_add(
    count: u8,
    upgraded: bool,
    entities: &mut Vec<Entity>,
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
) -> DispatchResult {
    let shiv = get_card(CardName::Shiv, upgraded);

    for _ in 0..count {
        let id_card = entities.len();
        entities.push(shiv);

        if id_hand.len() < MAX_SIZE_HAND {
            id_hand.push(id_card)
        } else {
            id_pile_discard.push(id_card)
        }
    }

    DispatchResult::Continue
}
