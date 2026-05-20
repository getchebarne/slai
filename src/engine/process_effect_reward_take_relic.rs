use strum::EnumCount;

use crate::entity::Entity;
use crate::types::RelicName;
use crate::types::RewardState;

pub fn process_effect_reward_take_relic(
    reward: &mut RewardState,
    entities: &[Entity],
    id_relics: &mut [Option<usize>; RelicName::COUNT],
) {
    if let Some(id) = reward.id_relic.take() {
        let name = entities[id].relic_name;
        id_relics[name as usize] = Some(id);
    }
}
