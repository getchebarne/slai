use crate::game::GameState;
use crate::types::Mode;

pub fn process_effect_target_clear(state: &mut GameState) {
    let Some(Mode::Combat {
        id_picked_monster, ..
    }) = state.mode_stack.last_mut()
    else {
        unreachable!("process_effect_target_clear outside Combat mode")
    };
    *id_picked_monster = None;
}
