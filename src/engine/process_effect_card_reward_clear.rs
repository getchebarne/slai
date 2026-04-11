use crate::engine::{HaltReason, ProcessEffectResult};
use crate::types::EntityId;

pub fn process_effect_card_reward_clear(card_rewards: &mut Vec<EntityId>) -> ProcessEffectResult {
    // Clear card rewards
    card_rewards.clear();

    // Card reward resolution always transitions back to map-node selection.
    ProcessEffectResult::Halt(HaltReason::AwaitMapNode)
}
