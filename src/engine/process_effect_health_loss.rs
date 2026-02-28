use crate::effect::Effect;
use crate::engine::ProcessEffectResult;
use crate::modifier::{ModifierKind, Modifiers, modifier_def, modifier_has, modifier_remove, modifier_stacks};
use crate::state::Vitals;
use crate::types::EntityId;

pub fn process_effect_health_loss(
    vitals: &mut Vitals,
    modifiers: &mut Modifiers,
    target: EntityId,
    amount: u16,
) -> ProcessEffectResult {
    vitals.health = vitals.health.saturating_sub(amount);

    let mut effects = Vec::new();

    if vitals.health == 0 {
        effects.push(Effect::Death { actor: target });
    } else if modifier_has(modifiers, ModifierKind::ModeShift) {
        let new_stacks =
            modifier_stacks(modifiers, ModifierKind::ModeShift) - amount as i16;
        if new_stacks < modifier_def(ModifierKind::ModeShift).stacks_min {
            modifier_remove(modifiers, ModifierKind::ModeShift);
            if target.0 != 0 {
                effects.push(Effect::MoveUpdate { monster: target });
            }
        } else {
            modifiers.stacks[ModifierKind::ModeShift as usize] = new_stacks;
        }
    }

    if effects.is_empty() {
        ProcessEffectResult::Pass
    } else {
        ProcessEffectResult::Continue {
            top: effects,
            bot: Vec::new(),
        }
    }
}
