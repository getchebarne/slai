use crate::effect::{Effect, EffectKind, Target};
use crate::engine::ProcessEffectResult;
use crate::modifier::{
    ModifierKind, Modifiers, modifier_def, modifier_has, modifier_remove, modifier_stacks,
};
use crate::state::Vitals;

pub fn process_effect_health_loss(
    vitals: &mut Vitals,
    modifiers: &mut Modifiers,
    target: usize,
    character: usize,
    amount: u16,
) -> ProcessEffectResult {
    vitals.health = vitals.health.saturating_sub(amount);
    let mut effects = Vec::new();

    // Death check
    if vitals.health == 0 {
        effects.push(Effect {
            kind: EffectKind::Death,
            source: None,
            target: Target::Direct(Some(target)),
        });

    // Modifier / ModeShift (damage reduces stacks, triggers move update on break)
    } else if modifier_has(modifiers, ModifierKind::ModeShift) {
        let new_stacks = modifier_stacks(modifiers, ModifierKind::ModeShift) - amount as i16;
        if new_stacks < modifier_def(ModifierKind::ModeShift).stacks_min {
            modifier_remove(modifiers, ModifierKind::ModeShift);
            if target != character {
                effects.push(Effect {
                    kind: EffectKind::MoveUpdate,
                    source: None,
                    target: Target::Direct(Some(target)),
                });
            }
        } else {
            modifiers.stacks[ModifierKind::ModeShift as usize] = new_stacks;
        }
    }

    if effects.is_empty() {
        ProcessEffectResult::Continue
    } else {
        ProcessEffectResult::AddAndContinue {
            top: effects,
            bot: Vec::new(),
        }
    }
}
