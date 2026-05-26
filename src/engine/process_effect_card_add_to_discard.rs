use crate::cards::get_card;
use crate::utils::push_entity;
use crate::game::GameState;
use crate::types::CardName;

pub fn process_effect_card_add_to_discard(
    state: &mut GameState,
    card_name: CardName,
    count: u8,
    upgraded: bool,
) {
    if count == 0 {
        return;
    }
    for _ in 0..count {
        let template = get_card(card_name, upgraded);
        let id_card = push_entity(&mut state.entities, template);
        state.id_pile_discard.push(id_card);
    }
}
