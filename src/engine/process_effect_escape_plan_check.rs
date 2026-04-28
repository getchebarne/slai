use std::collections::VecDeque;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::types::CardKind;

// EscapePlan post-draw check: if the last card drawn (set by CardDraw) is a
// Skill, gain `block`. Consumes `state.card_last_drawn` so it can't fire on
// stale state if it runs again later
pub fn process_effect_escape_plan_check(
    entities: &[Entity],
    id_character: usize,
    card_last_drawn: &mut Option<usize>,
    block: u16,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let id_card = match card_last_drawn.take() {
        Some(id) => id,
        None => return DispatchResult::Continue,
    };
    if entities[id_card].card_kind != CardKind::Skill {
        return DispatchResult::Continue;
    }
    queue.push_front(Effect {
        kind: EffectKind::BlockGain { amount: block },
        id_source: Some(id_character),
        target: Target::Direct(Some(id_character)),
    });
    DispatchResult::Continue
}
