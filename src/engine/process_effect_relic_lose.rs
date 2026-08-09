use crate::game::GameState;
use crate::types::RelicName;

// Unregisters the Relic; the entity stays orphaned in the arena
pub fn process_effect_relic_lose(state: &mut GameState, name: RelicName) {
    assert!(
        state.id_relics[name as usize].is_some(),
        "RelicLose without owning {name:?}"
    );
    state.id_relics[name as usize] = None;
}
