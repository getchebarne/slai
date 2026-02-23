use crate::consts::{FACTOR_VULN, FACTOR_WEAK};
use crate::effect::Effect;
use crate::engine::ProcessEffectResult;
use crate::modifier::{ModifierKind, Modifiers, modifier_has, modifier_stacks};
use crate::types::ActorId;

pub fn process_effect_damage_physical(
    source_mods: &Modifiers,
    target_mods: &Modifiers,
    target: ActorId,
    base: u16,
) -> ProcessEffectResult {
    let mut value = base as f32;

    // Apply modifiers. Order matters here
    if modifier_has(source_mods, ModifierKind::Strength) {
        value += modifier_stacks(source_mods, ModifierKind::Strength) as f32;
    }

    if modifier_has(source_mods, ModifierKind::Weak) {
        value *= FACTOR_WEAK;
    }

    if modifier_has(target_mods, ModifierKind::Vulnerable) {
        value *= FACTOR_VULN;
    }

    if modifier_has(source_mods, ModifierKind::DoubleDamage) {
        value *= 2.0;
    }

    let final_damage = value as u16;
    if final_damage > 0 {
        ProcessEffectResult::Continue {
            top: vec![Effect::DamageDeal {
                target,
                amount: final_damage,
            }],
            bot: Vec::new(),
        }
    } else {
        ProcessEffectResult::Pass
    }
}
