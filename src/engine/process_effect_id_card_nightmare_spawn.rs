use crate::consts::MAX_SIZE_HAND;
use crate::consts::NIGHTMARE_COPIES;
use crate::engine::entities_push;
use crate::game::GameState;

pub fn process_effect_id_card_nightmare_spawn(state: &mut GameState) {
    let id_template = state
        .id_card_nightmare
        .take()
        .expect("CardNightmareSpawn with no pending snapshot");

    let template = state.entities[id_template];
    for _ in 0..NIGHTMARE_COPIES {
        let id_card = entities_push(&mut state.entities, template);
        if state.id_hand.len() < MAX_SIZE_HAND {
            state.id_hand.push(id_card);
        } else {
            state.id_pile_discard.push(id_card);
        }
    }
}
