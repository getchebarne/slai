use std::collections::VecDeque;

use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::engine::DispatchResult;

pub fn process_effect_calculated_gamble(
    id_hand: &[usize],
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let num_cards = id_hand.len();
    // Draw is the LAST effect (runs after discards), so push it first
    // (push_front reverses order)
    effect_queue.push_front(Effect {
        kind: EffectKind::CardDraw {
            count: num_cards as u8,
        },
        id_source: None,
        target: Target::Direct(None),
    });
    // Discards in original order: iterate reverse, push_front
    for &id_card in id_hand.iter().rev() {
        effect_queue.push_front(Effect {
            kind: EffectKind::CardDiscard {
                source: DiscardSource::Explicit,
            },
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    }
    DispatchResult::Continue
}
