use crate::engine::DispatchResult;
use crate::entity::Entity;

// Silently remove a monster from combat: flag it dead WITHOUT firing the
// on-death hooks
pub fn process_effect_escape_monster(id_target: usize, entities: &mut [Entity]) -> DispatchResult {
    entities[id_target].dead = true;
    DispatchResult::Continue
}
