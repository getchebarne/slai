use crate::engine::ProcessEffectResult;
use crate::types::EntityId;

pub fn process_effect_card_reward_clear(card_rewards: &mut Vec<EntityId>) -> ProcessEffectResult {
    card_rewards.clear();
    ProcessEffectResult::Continue
}
