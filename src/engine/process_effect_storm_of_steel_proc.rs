use std::collections::VecDeque;

use crate::effect::{DiscardSource, Effect, EffectKind, Target};
use crate::engine::DispatchResult;

// Storm of Steel: discard the entire hand, then add 1 Shiv per discarded card
pub fn process_effect_storm_of_steel_proc(
    upgraded: bool,
    id_hand: &[usize],
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let n = id_hand.len() as u8;
    effect_queue.push_front(Effect {
        kind: EffectKind::ShivAdd { count: n, upgraded },
        id_source: None,
        target: Target::Direct(None),
    });
    for &id_card in id_hand {
        effect_queue.push_front(Effect::direct(
            EffectKind::CardDiscard {
                source: DiscardSource::Explicit,
            },
            None,
            Some(id_card),
        ));
    }
    DispatchResult::Continue
}
