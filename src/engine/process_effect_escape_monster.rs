use std::collections::VecDeque;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::engine::DispatchResult;
use crate::entity::Entity;

// Silently remove a monster from combat: flag it dead WITHOUT firing the
// on-death hook chain
pub fn process_effect_escape_monster(
    id_target: usize,
    id_monsters: &[usize],
    monster_count: u8,
    entities: &mut [Entity],
    escaped_this_combat: &mut bool,
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    entities[id_target].dead = true;
    *escaped_this_combat = true;

    let any_alive = id_monsters[..monster_count as usize]
        .iter()
        .any(|&id| !entities[id].dead);
    if !any_alive {
        effect_queue.clear();
        effect_queue.push_back(Effect {
            kind: EffectKind::CombatEnd,
            id_source: None,
            target: Target::Direct(None),
        });
    }
    DispatchResult::Continue
}
