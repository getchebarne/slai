use crate::game::GameState;
use crate::types::Combat;

pub fn process_effect_target_set(id_target: Option<usize>, state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_target_set outside the Combat frame"
    );
    let Combat {
        id_monster_picked, ..
    } = &mut state.combat;
    let id_target = id_target.expect("TargetSet requires id_target");
    *id_monster_picked = Some(id_target);
}
