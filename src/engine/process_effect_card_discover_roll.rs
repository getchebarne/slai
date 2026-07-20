use crate::cards::get_random_cards;
use crate::game::GameState;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::Mode;
use crate::utils::push_entity;

pub fn process_effect_card_discover_roll(
    state: &mut GameState,
    kind: CardKind,
    color: CardColor,
    count: u8,
) {
    let Mode::Combat(combat) = &mut state.mode else {
        unreachable!("process_effect_card_discover_roll outside Combat mode")
    };
    combat.id_discover.clear();

    let card_picks = get_random_cards(color, Some(kind), None, &[], count as usize, &mut state.rng);
    for card_pick in card_picks {
        let id = push_entity(&mut state.entities, card_pick);
        combat.id_discover.push(id);
    }
}
