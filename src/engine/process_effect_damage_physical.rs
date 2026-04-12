use crate::consts::{FACTOR_VULN, FACTOR_WEAK};
use crate::effect::{Effect, EffectKind, Target};
use crate::engine::ProcessEffectResult;
use crate::modifier::{ModifierKind, Modifiers, modifier_has, modifier_stacks};
use crate::types::EntityId;

pub fn process_effect_damage_physical(
    source_mods: &Modifiers,
    target_mods: &Modifiers,
    target: EntityId,
    base: u16,
) -> ProcessEffectResult {
    let mut value = base as f32;

    // Source modifiers
    if modifier_has(source_mods, ModifierKind::Strength) {
        value += modifier_stacks(source_mods, ModifierKind::Strength) as f32;
    }
    if modifier_has(source_mods, ModifierKind::Weak) {
        value *= FACTOR_WEAK;
    }
    if modifier_has(source_mods, ModifierKind::DoubleDamage) {
        value *= 2.0;
    }

    // Target modifiers
    if modifier_has(target_mods, ModifierKind::Vulnerable) {
        value *= FACTOR_VULN;
    }

    // Emit DamageDeal if positive
    let final_damage = value.max(0.0) as u16;
    if final_damage > 0 {
        ProcessEffectResult::AddAndContinue {
            top: vec![Effect {
                kind: EffectKind::DamageDeal {
                    amount: final_damage,
                },
                source: None,
                target: Target::Direct(Some(target)),
            }],
            bot: Vec::new(),
        }
    } else {
        ProcessEffectResult::Continue
    }
}
