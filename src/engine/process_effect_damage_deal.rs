use std::collections::VecDeque;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::types::Vitals;

pub fn process_effect_damage_deal(
    vitals: &mut Vitals,
    id_target: usize,
    amount: u16,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let damage_over_block = amount.saturating_sub(vitals.block);
    vitals.block = vitals.block.saturating_sub(amount);

    if damage_over_block > 0 {
        queue.push_front(Effect {
            kind: EffectKind::HealthLoss {
                amount: damage_over_block,
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
    }
    DispatchResult::Continue
}
