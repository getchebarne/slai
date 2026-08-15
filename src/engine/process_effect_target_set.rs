use crate::game::GameState;
use crate::types::Frame;
use crate::utils::frame_top_mut;

pub fn process_effect_target_set(id_target: Option<usize>, state: &mut GameState) {
    let Frame::Combat {
        id_picked_monster, ..
    } = frame_top_mut(&mut state.frame_stack)
    else {
        unreachable!("process_effect_target_set outside the Combat frame")
    };
    let id_target = id_target.expect("TargetSet requires id_target");
    *id_picked_monster = Some(id_target);
}
