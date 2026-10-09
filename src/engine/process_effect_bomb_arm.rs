use crate::game::GameState;

// Each Bomb ticks independently: its own fuse, its own damage, its own stamp
pub fn process_effect_bomb_arm(state: &mut GameState, turns: u8, damage: u16) {
    let seq = state.combat.modifier_seq_next;
    state.combat.modifier_seq_next += 1;
    state.combat.bombs.push((turns, damage, seq));
}
