use crate::cards::get_card;
use crate::entity::Entity;
use crate::types::CardName;
use crate::types::Phase;

pub fn process_effect_card_add_to_deck(
    card_name: CardName,
    upgraded: bool,
    entities: &mut Vec<Entity>,
    id_deck: &mut Vec<usize>,
) -> Option<Phase> {
    let id = entities.len();
    entities.push(get_card(card_name, upgraded));
    id_deck.push(id);
    None
}
