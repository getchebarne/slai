use std::collections::VecDeque;

use crate::effect::Effect;
use crate::engine::DispatchResult;
use crate::engine::try_complete_reward;
use crate::entity::Entity;
use crate::types::Phase;

pub fn process_effect_gold_reward_take(
    phase: &mut Phase,
    entities: &mut [Entity],
    id_character: usize,
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    if let Phase::Reward { gold, .. } = phase {
        if let Some(amount) = gold.take() {
            entities[id_character].character_gold =
                entities[id_character].character_gold.saturating_add(amount);
        }
    }
    try_complete_reward(phase, effect_queue);
    DispatchResult::Continue
}
