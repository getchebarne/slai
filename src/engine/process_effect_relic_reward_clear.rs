use std::collections::VecDeque;

use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::engine::DispatchResult;

pub fn process_effect_relic_reward_clear(
    id_relic_rewards: &mut Vec<usize>,
    id_card_rewards: &[usize],
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    id_relic_rewards.clear();
    if id_card_rewards.is_empty() {
        effect_queue.push_front(Effect {
            kind: EffectKind::RoomSelect,
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::NextRowRooms,
                selection: SelectionKind::Input { count: 1 },
            },
        });
    }
    DispatchResult::Continue
}
