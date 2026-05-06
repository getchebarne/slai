use std::collections::VecDeque;

use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::engine::DispatchResult;

pub fn process_effect_card_reward_clear(
    id_card_rewards: &mut Vec<usize>,
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    id_card_rewards.clear();
    effect_queue.push_front(Effect {
        kind: EffectKind::RoomSelect,
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::NextRowRooms,
            selection: SelectionKind::Input { count: 1 },
        },
    });
    DispatchResult::Continue
}
