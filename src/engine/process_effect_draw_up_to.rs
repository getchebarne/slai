use std::collections::VecDeque;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;

pub fn process_effect_draw_up_to(
    amount: u8,
    id_hand: &[usize],
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let num_cards_to_draw = amount.saturating_sub(id_hand.len() as u8);
    if num_cards_to_draw > 0 {
        effect_queue.push_front(Effect {
            kind: EffectKind::CardDraw {
                count: num_cards_to_draw,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }
    DispatchResult::Continue
}
