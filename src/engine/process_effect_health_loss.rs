use crate::effect::Effect;
use crate::engine::ProcessEffectResult;
use crate::modifier::{ModifierKind, modifier_def, modifier_has, modifier_remove, modifier_stacks};
use crate::state::Vitals;
use crate::types::EntityId;

pub fn process_effect_health_loss(
    vitals: &mut Vitals,
    target: EntityId,
    amount: u16,
    character_id: EntityId,
) -> ProcessEffectResult {
    vitals.health = vitals.health.saturating_sub(amount);

    let mut effects = Vec::new();

    if vitals.health == 0 {
        effects.push(Effect::Death { actor: target });
    } else if modifier_has(&vitals.modifiers, ModifierKind::ModeShift) {
        let new_stacks =
            modifier_stacks(&vitals.modifiers, ModifierKind::ModeShift) - amount as i16;
        if new_stacks < modifier_def(ModifierKind::ModeShift).stacks_min {
            modifier_remove(&mut vitals.modifiers, ModifierKind::ModeShift);
            if target != character_id {
                effects.push(Effect::MoveUpdate { monster: target });
            }
        } else {
            vitals.modifiers.stacks[ModifierKind::ModeShift as usize] = new_stacks;
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
