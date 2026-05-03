use std::collections::VecDeque;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::entity::Entity;

// Silently remove a monster from combat: flag it dead WITHOUT firing the
// on-death hook chain
pub fn process_effect_escape_monster(
    id_target: usize,
    id_monsters: &[usize],
    monster_count: u8,
    entities: &mut [Entity],
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    // Mark as dead
    entities[id_target].dead = true;

    let any_alive = id_monsters[..monster_count as usize]
        .iter()
        .any(|&id| !entities[id].dead);
    if !any_alive {
        queue.clear();
        queue.push_back(Effect {
            kind: EffectKind::CombatEnd,
            id_source: None,
            target: Target::Direct(None),
        });
    }
    DispatchResult::Continue
}
