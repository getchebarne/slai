use crate::game::GameState;
use crate::types::Mode;
use crate::utils::mode_top_mut;

pub fn process_effect_target_clear(state: &mut GameState) {
    let Mode::Combat {
        id_picked_monster, ..
    } = mode_top_mut(&mut state.mode_stack)
    else {
        unreachable!("process_effect_target_clear outside Combat mode")
    };
    *id_picked_monster = None;
}
