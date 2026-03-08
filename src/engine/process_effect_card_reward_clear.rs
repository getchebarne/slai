use crate::engine::ProcessEffectResult;
use crate::types::EntityId;

pub fn process_effect_card_reward_clear(card_rewards: &mut Vec<EntityId>) -> ProcessEffectResult {
    // Clear card rewards
    card_rewards.clear();

    // Continue
    ProcessEffectResult::Continue
}
