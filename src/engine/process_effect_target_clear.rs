use crate::game::GameState;
use crate::types::Frame;
use crate::utils::frame_top_mut;

pub fn process_effect_target_clear(state: &mut GameState) {
    let Frame::Combat {
        id_picked_monster, ..
    } = frame_top_mut(&mut state.frame_stack)
    else {
        unreachable!("process_effect_target_clear outside the Combat frame")
    };
    *id_picked_monster = None;
}
