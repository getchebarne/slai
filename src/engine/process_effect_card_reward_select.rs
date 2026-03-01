use crate::engine::ProcessEffectResult;
use crate::types::EntityId;

pub fn process_effect_card_reward_select(
    card_idx: usize,
    card_rewards: &mut Vec<EntityId>,
    deck: &mut Vec<EntityId>,
) -> ProcessEffectResult {
    let card_id = card_rewards[card_idx];
    deck.push(card_id);
    card_rewards.clear();
    ProcessEffectResult::Pass
}
