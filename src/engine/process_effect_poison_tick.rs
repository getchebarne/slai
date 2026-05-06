use std::collections::VecDeque;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::modifier::{ModifierKind, Modifiers, modifier_has, modifier_remove, modifier_stacks};

// Poison tick: deal HP loss equal to current Poison stacks, then decrement
// Poison by 1 (remove if it would hit 0). Fires at the start of the target's turn start
pub fn process_effect_poison_tick(
    modifiers: &mut Modifiers,
    id_target: usize,
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    if !modifier_has(modifiers, ModifierKind::Poison) {
        return DispatchResult::Continue;
    }
    // Get current stacks
    let stacks = modifier_stacks(modifiers, ModifierKind::Poison);

    // Update current stacks
    if stacks <= 1 {
        modifier_remove(modifiers, ModifierKind::Poison);
    } else {
        modifiers.stacks[ModifierKind::Poison as usize] = stacks - 1;
    }

    // Push health loss effect
    effect_queue.push_front(Effect {
        kind: EffectKind::HealthLoss {
            amount: stacks as u16,
        },
        id_source: None,
        target: Target::Direct(Some(id_target)),
    });
    DispatchResult::Continue
}
