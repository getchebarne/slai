use crate::cards::get_card;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::CardPile;
use crate::utils::place_card;
use crate::utils::push_entity;

pub fn process_effect_card_add(
    state: &mut GameState,
    card_name: CardName,
    pile: CardPile,
    count: u16,
    upgraded: bool,
) {
    for _ in 0..count {
        let card = get_card(card_name, upgraded);
        let id_card = push_entity(&mut state.entities, card);
        place_card(state, id_card, pile);
    }
}
