use std::collections::VecDeque;

use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::engine::DispatchResult;

pub fn process_effect_card_reward_clear(
    id_card_rewards: &mut Vec<usize>,
    id_relic_rewards: &[usize],
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    id_card_rewards.clear();
    if id_relic_rewards.is_empty() {
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
