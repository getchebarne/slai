use crate::consts::MAX_SIZE_HAND;
use crate::consts::NIGHTMARE_COPIES;
use crate::game::GameState;
use crate::types::Mode;
use crate::utils::push_entity;

pub fn process_effect_card_nightmare_spawn(state: &mut GameState) {
    let Mode::Combat {
        id_hand,
        id_pile_discard,
        id_card_nightmare,
        ..
    } = &mut state.mode
    else {
        unreachable!("process_effect_card_nightmare_spawn outside Combat mode")
    };
    let id_template = id_card_nightmare
        .take()
        .expect("CardNightmareSpawn with no pending snapshot");

    let template = state.entities[id_template];
    for _ in 0..NIGHTMARE_COPIES {
        let id_card = push_entity(&mut state.entities, template);
        if id_hand.len() < MAX_SIZE_HAND {
            id_hand.push(id_card);
        } else {
            id_pile_discard.push(id_card);
        }
    }
}
