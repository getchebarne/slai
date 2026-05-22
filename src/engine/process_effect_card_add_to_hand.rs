use crate::cards::get_card;
use crate::entity::add_card_to_hand_or_discard;
use crate::game::GameState;
use crate::types::CardName;

pub fn process_effect_card_add_to_hand(
    state: &mut GameState,
    card_name: CardName,
    count: u16,
    upgraded: bool,
) {
    if count == 0 {
        return;
    }
    for _ in 0..count {
        let card = get_card(card_name, upgraded);
        add_card_to_hand_or_discard(
            &mut state.entities,
            &mut state.id_hand,
            &mut state.id_pile_discard,
            card,
        );
    }
}
