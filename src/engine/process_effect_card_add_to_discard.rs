use crate::cards::get_card;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::Mode;
use crate::utils::push_entity;

pub fn process_effect_card_add_to_discard(
    state: &mut GameState,
    card_name: CardName,
    count: u8,
    upgraded: bool,
) {
    let Mode::Combat(combat) = &mut state.mode else {
        unreachable!("process_effect_card_add_to_discard outside Combat mode")
    };
    if count == 0 {
        return;
    }
    for _ in 0..count {
        let template = get_card(card_name, upgraded);
        let id_card = push_entity(&mut state.entities, template);
        combat.id_pile_discard.push(id_card);
    }
}
