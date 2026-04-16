use std::collections::VecDeque;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::modifier::{
    ModifierKind, Modifiers, modifier_def, modifier_has, modifier_remove, modifier_stacks,
};
use crate::types::Vitals;

pub fn process_effect_health_loss(
    vitals: &mut Vitals,
    modifiers: &mut Modifiers,
    target: usize,
    character: usize,
    amount: u16,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    vitals.health = vitals.health.saturating_sub(amount);

    if vitals.health == 0 {
        queue.push_front(Effect {
            kind: EffectKind::Death,
            source: None,
            target: Target::Direct(Some(target)),
        });
    } else if modifier_has(modifiers, ModifierKind::ModeShift) {
        // ModeShift: damage reduces stacks, triggers move update on break.
        let new_stacks = modifier_stacks(modifiers, ModifierKind::ModeShift) - amount as i16;
        if new_stacks < modifier_def(ModifierKind::ModeShift).stacks_min {
            modifier_remove(modifiers, ModifierKind::ModeShift);
            if target != character {
                queue.push_front(Effect {
                    kind: EffectKind::MoveUpdate,
                    source: None,
                    target: Target::Direct(Some(target)),
                });
            }
        } else {
            modifiers.stacks[ModifierKind::ModeShift as usize] = new_stacks;
        }
    }

    DispatchResult::Continue
}
