use crate::game::GameState;
use crate::types::Mode;
use crate::utils::mode_top_mut;

pub fn process_effect_target_set(id_target: Option<usize>, state: &mut GameState) {
    let Mode::Combat {
        id_picked_monster, ..
    } = mode_top_mut(&mut state.mode_stack)
    else {
        unreachable!("process_effect_target_set outside Combat mode")
    };
    let id_target = id_target.expect("TargetSet requires id_target");
    *id_picked_monster = Some(id_target);
}
