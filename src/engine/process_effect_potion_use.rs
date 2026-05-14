use std::collections::VecDeque;

use crate::effect::Effect;
use crate::engine::DispatchResult;
use crate::entity::Entity;

pub fn process_effect_potion_use(
    id_potion: usize,
    entities: &[Entity],
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let potion = &entities[id_potion];
    for eff in potion.potion_effects.iter().rev() {
        effect_queue.push_front(Effect {
            id_source: Some(id_potion),
            ..*eff
        });
    }
    DispatchResult::Continue
}
