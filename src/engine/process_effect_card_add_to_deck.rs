use crate::cards::get_card;
use crate::engine::entities_push;
use crate::game::GameState;
use crate::types::CardName;

pub fn process_effect_card_add_to_deck(
    state: &mut GameState,
    card_name: CardName,
    upgraded: bool,
) {
    let id = entities_push(&mut state.entities, get_card(card_name, upgraded));
    state.id_deck.push(id);
}
