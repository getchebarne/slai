use crate::utils::push_entity;
use crate::game::GameState;

pub fn process_effect_id_card_nightmare_pick(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("CardNightmarePick requires id_target");
    let card = state.entities[id_target];
    let id = push_entity(&mut state.entities, card);
    state.id_card_nightmare = Some(id);
}
