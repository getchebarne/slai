use crate::entity::Entity;
use crate::types::RewardState;

pub fn process_effect_reward_take_gold(
    reward: &mut RewardState,
    entities: &mut [Entity],
    id_character: usize,
) {
    if let Some(amount) = reward.gold.take() {
        entities[id_character].character_gold =
            entities[id_character].character_gold.saturating_add(amount);
    }
}
