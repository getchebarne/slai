use std::collections::VecDeque;

use rand::Rng;

use crate::effect::Effect;
use crate::types::Phase;

pub fn process_effect_roll_d100_branch(
    chance: u8,
    on_lt: &'static [Effect],
    on_ge: &'static [Effect],
    id_source: Option<usize>,
    rng: &mut impl Rng,
    effect_queue: &mut VecDeque<Effect>,
) -> Option<Phase> {
    let roll = rng.random_range(0..100) as u8;
    let branch = if roll < chance { on_lt } else { on_ge };
    for effect in branch.iter().rev() {
        effect_queue.push_front(Effect {
            id_source,
            ..*effect
        });
    }
    None
}
