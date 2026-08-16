use crate::cards::get_random_cards;
use crate::game::GameState;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::Combat;
use crate::utils::push_entity;

pub fn process_effect_card_discover_roll(
    state: &mut GameState,
    kind: Option<CardKind>,
    color: CardColor,
    exclude: &[CardName],
    count: u8,
) {
    assert!(
        state.combat.active,
        "process_effect_card_discover_roll outside the Combat frame"
    );
    let Combat {
        id_card_discover, ..
    } = &mut state.combat;
    id_card_discover.clear();

    let card_picks = get_random_cards(color, kind, None, exclude, count as usize, &mut state.rng);
    for card_pick in card_picks {
        let id = push_entity(&mut state.entities, card_pick);
        id_card_discover.push(id);
    }
}
