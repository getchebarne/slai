use crate::consts::NIGHTMARE_COPIES;
use crate::game::GameState;
use crate::types::CardPile;
use crate::utils::place_card;
use crate::utils::push_entity;

pub fn process_effect_card_nightmare_spawn(state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_card_nightmare_spawn outside the Combat frame"
    );
    assert!(
        !state.combat.id_card_nightmares.is_empty(),
        "CardNightmareSpawn with no pending snapshot"
    );

    // Each pending Nightmare adds its copies in play order, so the first fills the hand first
    for idx in 0..state.combat.id_card_nightmares.len() {
        let card_template = state.entities[state.combat.id_card_nightmares[idx]];
        for _ in 0..NIGHTMARE_COPIES {
            let id_card = push_entity(&mut state.entities, card_template);
            place_card(state, id_card, CardPile::Hand);
        }
    }
    state.combat.id_card_nightmares.clear();
}
