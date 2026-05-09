use std::collections::VecDeque;

use strum::EnumCount;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::types::RelicName;

pub fn process_effect_relic_reward_select(
    id_relic: usize,
    entities: &[Entity],
    id_relics: &mut [Option<usize>; RelicName::COUNT],
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let name = entities[id_relic].relic_name;
    id_relics[name as usize] = Some(id_relic);
    effect_queue.push_front(Effect {
        kind: EffectKind::RelicRewardClear,
        id_source: None,
        target: Target::Direct(None),
    });
    DispatchResult::Continue
}
