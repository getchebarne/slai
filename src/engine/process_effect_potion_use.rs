use std::collections::VecDeque;

use crate::effect::Effect;
use crate::entity::Entity;
use crate::types::Phase;

pub fn process_effect_potion_use(
    id_potion: usize,
    entities: &[Entity],
    effect_queue: &mut VecDeque<Effect>,
) -> Option<Phase> {
    let potion = &entities[id_potion];
    for effect in potion.potion_effects.iter().rev() {
        effect_queue.push_front(Effect {
            id_source: Some(id_potion),
            ..*effect
        });
    }
    None
}
