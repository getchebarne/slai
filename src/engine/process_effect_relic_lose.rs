use crate::game::GameState;

// Unregisters the targeted Relic; the entity stays orphaned in the arena
pub fn process_effect_relic_lose(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("RelicLose requires id_target");
    let name = state.entities[id_target].relic_name;
    assert!(
        state.id_relics[name as usize] == Some(id_target),
        "RelicLose without owning {name:?}"
    );
    state.id_relics[name as usize] = None;
}
