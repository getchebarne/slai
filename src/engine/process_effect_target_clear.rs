use crate::game::GameState;

// The tail of every play: the shared Monster target is released
pub fn process_effect_target_clear(state: &mut GameState) {
    state.combat.id_monster_target = None;
}
