use std::collections::VecDeque;

use rand::Rng;

use crate::cards::random_curse;
use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;

pub fn process_effect_curse_add(
    rng: &mut impl Rng,
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let curse = random_curse(rng);
    effect_queue.push_front(Effect {
        kind: EffectKind::CardAddSpecific {
            card_name: curse,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    });
    DispatchResult::Continue
}
