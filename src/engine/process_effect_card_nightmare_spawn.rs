use crate::consts::NIGHTMARE_COPIES;
use crate::game::GameState;
use crate::types::CardPile;
use crate::types::Mode;
use crate::utils::place_card;
use crate::utils::push_entity;

pub fn process_effect_card_nightmare_spawn(state: &mut GameState) {
    let Some(Mode::Combat {
        id_card_nightmare, ..
    }) = state.mode_stack.last_mut()
    else {
        unreachable!("process_effect_card_nightmare_spawn outside Combat mode")
    };
    let id_template = id_card_nightmare
        .take()
        .expect("CardNightmareSpawn with no pending snapshot");

    let template = state.entities[id_template];
    for _ in 0..NIGHTMARE_COPIES {
        let id_card = push_entity(&mut state.entities, template);
        place_card(state, id_card, CardPile::Hand);
    }
}
