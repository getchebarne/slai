use crate::game::GameState;
use crate::types::Combat;

pub fn process_effect_target_clear(state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_target_clear outside the Combat frame"
    );
    let Combat {
        id_monster_picked, ..
    } = &mut state.combat;
    *id_monster_picked = None;
}
