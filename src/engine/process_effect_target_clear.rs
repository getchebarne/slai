use crate::game::GameState;
use crate::types::Mode;

pub fn process_effect_target_clear(state: &mut GameState) {
    let Mode::Combat {
        id_picked_monster, ..
    } = &mut state.mode
    else {
        unreachable!("process_effect_target_clear outside Combat mode")
    };
    *id_picked_monster = None;
}
