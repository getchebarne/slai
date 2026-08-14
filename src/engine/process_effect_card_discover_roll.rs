use crate::cards::get_random_cards;
use crate::game::GameState;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::Frame;
use crate::utils::frame_top_mut;
use crate::utils::push_entity;

pub fn process_effect_card_discover_roll(
    state: &mut GameState,
    kind: Option<CardKind>,
    color: CardColor,
    exclude: &[CardName],
    count: u8,
) {
    let Frame::Combat { id_discover, .. } = frame_top_mut(&mut state.frame_stack) else {
        unreachable!("process_effect_card_discover_roll outside the Combat frame")
    };
    id_discover.clear();

    let card_picks = get_random_cards(color, kind, None, exclude, count as usize, &mut state.rng);
    for card_pick in card_picks {
        let id = push_entity(&mut state.entities, card_pick);
        id_discover.push(id);
    }
}
