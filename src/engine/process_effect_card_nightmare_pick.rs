use crate::game::GameState;
use crate::types::Frame;
use crate::utils::frame_top_mut;
use crate::utils::push_entity;

pub fn process_effect_card_nightmare_pick(id_target: Option<usize>, state: &mut GameState) {
    let Frame::Combat {
        id_card_nightmare, ..
    } = frame_top_mut(&mut state.frame_stack)
    else {
        unreachable!("process_effect_card_nightmare_pick outside the Combat frame")
    };
    let id_target = id_target.expect("CardNightmarePick requires id_target");
    let card = state.entities[id_target];
    let id = push_entity(&mut state.entities, card);
    *id_card_nightmare = Some(id);
}
