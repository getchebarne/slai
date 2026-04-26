use std::collections::VecDeque;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;

// Storm of Steel: discard the entire hand, then add 1 Shiv per discarded
// card. push_front order: ShivAdd first (so it ends up behind the discards
// in the queue), then a Direct CardDiscard per current hand card.
pub fn process_effect_storm_of_steel_proc(
    upgraded: bool,
    id_hand: &[usize],
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let n = id_hand.len() as u8;
    queue.push_front(Effect {
        kind: EffectKind::ShivAdd { count: n, upgraded },
        id_source: None,
        target: Target::Direct(None),
    });
    for &id_card in id_hand {
        queue.push_front(Effect::direct(EffectKind::CardDiscard, None, Some(id_card)));
    }
    DispatchResult::Continue
}
