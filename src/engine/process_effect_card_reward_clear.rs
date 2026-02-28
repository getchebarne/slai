use crate::cards::Card;
use crate::engine::ProcessEffectResult;

pub fn process_effect_card_reward_clear(card_rewards: &mut Vec<Card>) -> ProcessEffectResult {
    card_rewards.clear();
    ProcessEffectResult::Pass
}
