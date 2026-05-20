use crate::types::RewardState;

pub fn process_effect_reward_skip(reward: &mut RewardState) {
    reward.id_cards.clear();
    reward.id_relic = None;
    reward.id_potion = None;
    reward.gold = None;
}
