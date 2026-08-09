use crate::game::GameState;
use crate::types::RelicName;

// Unregisters the Relic; the entity stays orphaned in the arena
pub fn process_effect_relic_lose(state: &mut GameState, name: RelicName) {
    let id = state.id_relics[name as usize]
        .unwrap_or_else(|| panic!("RelicLose without owning {name:?}"));
    state.id_relics_order.retain(|&i| i != id);
    state.id_relics[name as usize] = None;
}
