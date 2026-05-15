use crate::cards::get_card;
use crate::entity::Entity;
use crate::types::CardName;
use crate::types::Phase;

pub fn process_effect_card_add_to_discard(
    card_name: CardName,
    count: u8,
    upgraded: bool,
    entities: &mut Vec<Entity>,
    id_pile_discard: &mut Vec<usize>,
) -> Option<Phase> {
    if count == 0 {
        return None;
    }
    for _ in 0..count {
        let template = get_card(card_name, upgraded);
        let id_card = entities.len();
        entities.push(template);
        id_pile_discard.push(id_card);
    }
    None
}
