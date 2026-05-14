use std::collections::VecDeque;

use crate::effect::Effect;
use crate::engine::DispatchResult;
use crate::engine::try_complete_reward;
use crate::types::Phase;

pub fn process_effect_potion_reward_clear(
    phase: &mut Phase,
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    if let Phase::Reward { id_potion, .. } = phase {
        *id_potion = None;
    }
    try_complete_reward(phase, effect_queue);
    DispatchResult::Continue
}
