use std::collections::VecDeque;

use crate::consts::MAX_RELICS;
use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;

pub fn process_effect_relic_reward_select(
    id_relic: usize,
    id_relics: &mut [usize; MAX_RELICS],
    relic_count: &mut u8,
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    id_relics[*relic_count as usize] = id_relic;
    *relic_count += 1;
    effect_queue.push_front(Effect {
        kind: EffectKind::RelicRewardClear,
        id_source: None,
        target: Target::Direct(None),
    });
    DispatchResult::Continue
}
