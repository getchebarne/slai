use std::collections::VecDeque;

use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::engine::DispatchResult;
use crate::types::Phase;

pub fn process_effect_reward_skip(
    phase: &mut Phase,
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    if let Phase::Reward { id_cards, id_relic, id_potion, gold } = phase {
        id_cards.clear();
        *id_relic = None;
        *id_potion = None;
        *gold = None;
    }
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
