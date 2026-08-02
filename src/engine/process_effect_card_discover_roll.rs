use crate::cards::get_random_cards;
use crate::game::GameState;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::Mode;
use crate::utils::push_entity;

pub fn process_effect_card_discover_roll(
    state: &mut GameState,
    kind: Option<CardKind>,
    color: CardColor,
    exclude: &[CardName],
    count: u8,
) {
    let Some(Mode::Combat { id_discover, .. }) = state.mode_stack.last_mut() else {
        unreachable!("process_effect_card_discover_roll outside Combat mode")
    };
    id_discover.clear();

    let card_picks = get_random_cards(color, kind, None, exclude, count as usize, &mut state.rng);
    for card_pick in card_picks {
        let id = push_entity(&mut state.entities, card_pick);
        id_discover.push(id);
    }
}
