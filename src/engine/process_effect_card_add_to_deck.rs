use crate::cards::get_card;
use crate::game::GameState;
use crate::types::CardName;
use crate::utils::push_entity;

pub fn process_effect_card_add_to_deck(state: &mut GameState, card_name: CardName, upgraded: bool) {
    let id = push_entity(&mut state.entities, get_card(card_name, upgraded));
    state.id_deck.push(id);
}
