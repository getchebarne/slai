use std::collections::VecDeque;

use crate::consts::{FACTOR_VULN, FACTOR_WEAK};
use crate::effect::{DamageCondition, Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::modifier::{ModifierKind, modifier_has, modifier_stacks};

// Unified physical-damage handler. `condition` selects whether the hit is
// unconditional (Always) or gated on the target being Poisoned (Bane).
//
// IfPoisoned bails (no damage, no Thorns) when the target is dead or lacks
// Poison; otherwise both branches run the same scaling pipeline:
// Strength + Weak + DoubleDamage on source, Vulnerable + Intangible on
// target, Thorns reflect, then push DamageDeal
pub fn process_effect_damage_physical(
    entities: &[Entity],
    id_source: Option<usize>,
    id_target: usize,
    amount: u16,
    condition: DamageCondition,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let target = &entities[id_target];
    if let DamageCondition::IfPoisoned = condition {
        if target.dead || !modifier_has(&target.modifiers, ModifierKind::Poison) {
            return DispatchResult::Continue;
        }
    }

    let mods_source = &entities[id_source.unwrap()].modifiers;
    let mods_target = &target.modifiers;
    let mut value = amount as f32;

    // Source modifiers
    if modifier_has(mods_source, ModifierKind::Strength) {
        value += modifier_stacks(mods_source, ModifierKind::Strength) as f32;
    }
    if modifier_has(mods_source, ModifierKind::Weak) {
        value *= FACTOR_WEAK;
    }
    if modifier_has(mods_source, ModifierKind::DoubleDamage) {
        value *= 2.0;
    }

    // Target modifiers
    if modifier_has(mods_target, ModifierKind::Vulnerable) {
        value *= FACTOR_VULN;
    }

    // Intangible
    if modifier_has(mods_target, ModifierKind::Intangible) && value > 1.0 {
        value = 1.0;
    }

    // Thorns: triggers per attack instance regardless of damage actually dealt
    if let Some(id_source) = id_source {
        if id_source != id_target && modifier_has(mods_target, ModifierKind::Thorns) {
            let stacks = modifier_stacks(mods_target, ModifierKind::Thorns);
            queue.push_front(Effect {
                kind: EffectKind::DamageDeal {
                    amount: stacks as u16,
                },
                id_source: None,
                target: Target::Direct(Some(id_source)),
            });
        }
    }

    let final_damage = value.max(0.0) as u16;
    if final_damage > 0 {
        queue.push_front(Effect {
            kind: EffectKind::DamageDeal {
                amount: final_damage,
            },
            id_source,
            target: Target::Direct(Some(id_target)),
        });
    }
    DispatchResult::Continue
}
