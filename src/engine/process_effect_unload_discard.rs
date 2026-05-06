use std::collections::VecDeque;

use crate::effect::{DiscardSource, Effect, EffectKind};
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::types::CardKind;

// Unload: discard every non-Attack card from hand
pub fn process_effect_unload_discard(
    entities: &[Entity],
    id_hand: &[usize],
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    for &id_card in id_hand {
        if entities[id_card].card_kind != CardKind::Attack {
            effect_queue.push_front(Effect::direct(
                EffectKind::CardDiscard {
                    source: DiscardSource::Explicit,
                },
                None,
                Some(id_card),
            ));
        }
    }
    DispatchResult::Continue
}
