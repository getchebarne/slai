use crate::game::GameState;

pub fn process_effect_target_clear(state: &mut GameState) {
    state.id_monster_picked = None;
}
