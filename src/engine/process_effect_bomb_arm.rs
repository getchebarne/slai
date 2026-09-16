use crate::game::GameState;

// Each Bomb is its own power in the source: its own fuse, its own damage
pub fn process_effect_bomb_arm(state: &mut GameState, turns: u8, damage: u16) {
    state.combat.bombs.push((turns, damage));
}
