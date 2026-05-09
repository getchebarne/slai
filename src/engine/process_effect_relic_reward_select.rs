use std::collections::VecDeque;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::types::N_RELICS;

pub fn process_effect_relic_reward_select(
    id_relic: usize,
    entities: &[Entity],
    relics_active: &mut u128,
    id_relics: &mut [usize; N_RELICS],
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let name = entities[id_relic].relic_name;
    *relics_active |= 1u128 << name as u32;
    id_relics[name as usize] = id_relic;
    effect_queue.push_front(Effect {
        kind: EffectKind::RelicRewardClear,
        id_source: None,
        target: Target::Direct(None),
    });
    DispatchResult::Continue
}
