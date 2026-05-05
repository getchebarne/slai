use std::collections::VecDeque;

use crate::consts::{FACTOR_VULN, FACTOR_WEAK};
use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::modifier::{ModifierKind, modifier_has, modifier_stacks};

// Unified physical-damage handler. `if_poisoned` gates the hit: when true
// (Bane), the handler bails (no damage, no Thorns) unless the target has
// Poison; when false, the hit always lands. Both branches share the same
// scaling pipeline: Strength + Weak + DoubleDamage on the actor, Vulnerable +
// Intangible on target, Thorns reflect, then push DamageDeal
pub fn process_effect_damage_physical(
    entities: &[Entity],
    id_source: Option<usize>,
    id_actor: usize,
    id_target: usize,
    amount: u16,
    if_poisoned: bool,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let target = &entities[id_target];
    if if_poisoned && (target.dead || !modifier_has(&target.modifiers, ModifierKind::Poison)) {
        return DispatchResult::Continue;
    }

    let mods_actor = &entities[id_actor].modifiers;
    let mods_target = &target.modifiers;
    let mut value = amount as f32;

    // Actor modifiers
    if modifier_has(mods_actor, ModifierKind::Strength) {
        value += modifier_stacks(mods_actor, ModifierKind::Strength) as f32;
    }
    if modifier_has(mods_actor, ModifierKind::Weak) {
        value *= FACTOR_WEAK;
    }
    if modifier_has(mods_actor, ModifierKind::DoubleDamage) {
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
    if id_actor != id_target && modifier_has(mods_target, ModifierKind::Thorns) {
        let stacks = modifier_stacks(mods_target, ModifierKind::Thorns);
        queue.push_front(Effect {
            kind: EffectKind::DamageDeal {
                amount: stacks as u16,
            },
            id_source: None,
            target: Target::Direct(Some(id_actor)),
        });
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
