use std::collections::VecDeque;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;

pub fn process_effect_draw_up_to(
    target: u8,
    id_hand: &[usize],
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let n = target.saturating_sub(id_hand.len() as u8);
    if n > 0 {
        queue.push_front(Effect {
            kind: EffectKind::CardDraw { count: n },
            id_source: None,
            target: Target::Direct(None),
        });
    }
    DispatchResult::Continue
}
