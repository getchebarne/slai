use strum::EnumCount;

use crate::entity::Entity;
use crate::types::Phase;
use crate::types::RelicName;

pub fn process_effect_reward_take_relic(
    phase: &mut Phase,
    entities: &[Entity],
    id_relics: &mut [Option<usize>; RelicName::COUNT],
) -> Option<Phase> {
    if let Phase::Reward { id_relic, .. } = phase {
        if let Some(id) = id_relic.take() {
            let name = entities[id].relic_name;
            id_relics[name as usize] = Some(id);
        }
    }
    None
}
