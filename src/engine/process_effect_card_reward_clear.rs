use std::collections::VecDeque;

use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::engine::DispatchResult;

pub fn process_effect_card_reward_clear(
    card_rewards: &mut Vec<usize>,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    card_rewards.clear();
    queue.push_front(Effect {
        kind: EffectKind::RoomSelect,
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::NextRowRooms,
            selection: SelectionKind::Input { count: 1 },
        },
    });
    DispatchResult::Continue
}
