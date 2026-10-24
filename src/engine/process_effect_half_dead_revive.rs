use crate::game::GameState;

// A half-dead Monster's revive starts here: from now on hits, Powers, Cards and random rolls reach it again
pub fn process_effect_half_dead_revive(id_target: Option<usize>, state: &mut GameState) {
    let id_monster = id_target.expect("HalfDeadRevive requires id_target");
    state.entities[id_monster].monster_half_dead = false;
}
