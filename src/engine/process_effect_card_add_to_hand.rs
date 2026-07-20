use crate::cards::get_card;
use crate::entity::add_card_to_hand_or_discard;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::Mode;

pub fn process_effect_card_add_to_hand(
    state: &mut GameState,
    card_name: CardName,
    count: u16,
    upgraded: bool,
) {
    let Mode::Combat(combat) = &mut state.mode else {
        unreachable!("process_effect_card_add_to_hand outside Combat mode")
    };
    if count == 0 {
        return;
    }
    for _ in 0..count {
        let card = get_card(card_name, upgraded);
        add_card_to_hand_or_discard(
            &mut state.entities,
            &mut combat.id_hand,
            &mut combat.id_pile_discard,
            card,
        );
    }
}
