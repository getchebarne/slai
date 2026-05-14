use crate::cards::get_card;
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::types::CardName;

pub fn process_effect_card_add_to_deck(
    card_name: CardName,
    upgraded: bool,
    entities: &mut Vec<Entity>,
    id_deck: &mut Vec<usize>,
) -> DispatchResult {
    let id = entities.len();
    entities.push(get_card(card_name, upgraded));
    id_deck.push(id);
    DispatchResult::Continue
}
