use std::collections::VecDeque;

use strum::EnumCount;

use crate::effect::Effect;
use crate::engine::DispatchResult;
use crate::engine::try_complete_reward;
use crate::entity::Entity;
use crate::types::Phase;
use crate::types::RelicName;

pub fn process_effect_relic_reward_select(
    phase: &mut Phase,
    entities: &[Entity],
    id_relics: &mut [Option<usize>; RelicName::COUNT],
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    if let Phase::Reward { id_relic, .. } = phase {
        if let Some(id) = id_relic.take() {
            let name = entities[id].relic_name;
            id_relics[name as usize] = Some(id);
        }
    }
    try_complete_reward(phase, effect_queue);
    DispatchResult::Continue
}
