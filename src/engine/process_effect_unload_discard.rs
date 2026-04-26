use std::collections::VecDeque;

use crate::effect::{Effect, EffectKind};
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::types::CardKind;

// Unload: discard every non-Attack card from hand. Forced (no player input).
// Same emission shape as CalculatedGamble but filtered.
pub fn process_effect_unload_discard(
    entities: &[Entity],
    id_hand: &[usize],
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    for &id_card in id_hand {
        if entities[id_card].card_kind != CardKind::Attack {
            queue.push_front(Effect::direct(EffectKind::CardDiscard, None, Some(id_card)));
        }
    }
    DispatchResult::Continue
}
