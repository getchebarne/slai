use crate::types::RewardState;

pub fn process_effect_card_reward_clear(reward: &mut RewardState) {
    reward.id_cards.clear();
}
