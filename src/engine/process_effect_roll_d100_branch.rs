use rand::Rng;

use crate::effect::Effect;
use crate::game::GameState;

pub fn process_effect_roll_d100_branch(
    id_source: Option<usize>,
    state: &mut GameState,
    chance: u8,
    on_lt: &'static [Effect],
    on_ge: &'static [Effect],
) {
    let roll = state.rng.random_range(0..100) as u8;
    let branch = if roll < chance { on_lt } else { on_ge };
    for effect in branch.iter().rev() {
        state.effect_queue.push_front(Effect {
            id_source,
            ..*effect
        });
    }
}
