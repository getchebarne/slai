use crate::game::GameState;
use crate::types::Mode;

pub fn process_effect_target_set(id_target: Option<usize>, state: &mut GameState) {
    let Some(Mode::Combat {
        id_picked_monster, ..
    }) = state.mode_stack.last_mut()
    else {
        unreachable!("process_effect_target_set outside Combat mode")
    };
    let id_target = id_target.expect("TargetSet requires id_target");
    *id_picked_monster = Some(id_target);
}
