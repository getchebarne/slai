use crate::game::GameState;

pub fn process_effect_target_set(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("TargetSet requires id_target");
    state.id_picked_monster = Some(id_target);
}
