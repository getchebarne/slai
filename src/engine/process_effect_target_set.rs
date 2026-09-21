use crate::game::GameState;

// The Monster a play's effects share; one play at a time holds it
pub fn process_effect_target_set(id_target: Option<usize>, state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_target_set outside the Combat frame"
    );
    assert!(
        state.combat.id_monster_target.is_none(),
        "TargetSet over a held target: nested targeted plays are not supported"
    );
    state.combat.id_monster_target = Some(id_target.expect("TargetSet requires id_target"));
}
