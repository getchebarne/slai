use std::collections::VecDeque;

use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::types::CardName;

// Storm of Steel: discard the entire hand, then add 1 Shiv per discarded card
pub fn process_effect_storm_of_steel_proc(
    upgraded: bool,
    id_hand: &[usize],
    effect_queue: &mut VecDeque<Effect>,
) {
    let n = id_hand.len() as u16;
    effect_queue.push_front(Effect {
        kind: EffectKind::CardAddToHand {
            card_name: CardName::Shiv,
            count: n,
            upgraded,
        },
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
}
