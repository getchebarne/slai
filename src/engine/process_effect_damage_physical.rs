use std::collections::VecDeque;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::engine::get_id_actor;
use crate::entity::Entity;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::modifier::modifier_stacks;
use crate::utils::scale_attack_damage;

// Unified physical-damage handler. `if_poisoned` gates the hit: when true
// (Bane), the handler bails (no damage, no Thorns) unless the target has
// Poison; when false, the hit always lands. Scaling: Strength + Vigor + Weak
// on actor, Vulnerable on target via `scale_attack_damage` (shared with the
// FFI intent view), then DoubleDamage ×2, Intangible clamp, Thorns reflect,
// finally push DamageDeal
pub fn process_effect_damage_physical(
    entities: &[Entity],
    id_source: usize,
    id_character: usize,
    id_target: usize,
    amount: u16,
    if_poisoned: bool,
    effect_queue: &mut VecDeque<Effect>,
) {
    let target = &entities[id_target];
    if if_poisoned && (target.dead || !modifier_has(&target.modifiers, ModifierKind::Poison)) {
        return;
    }

    let id_actor = get_id_actor(entities, id_character, id_source);
    let mods_actor = &entities[id_actor].modifiers;
    let mods_target = &target.modifiers;

    // Vigor folds into the base
    let base_with_vigor = amount
        + if modifier_has(mods_actor, ModifierKind::Vigor) {
            modifier_stacks(mods_actor, ModifierKind::Vigor).max(0) as u16
        } else {
            0
        };
    let str_stacks = if modifier_has(mods_actor, ModifierKind::Strength) {
        modifier_stacks(mods_actor, ModifierKind::Strength)
    } else {
        0
    };
    let mut final_damage = scale_attack_damage(
        base_with_vigor,
        str_stacks,
        modifier_has(mods_actor, ModifierKind::Weak),
        modifier_has(mods_target, ModifierKind::Vulnerable),
    );
    if modifier_has(mods_actor, ModifierKind::DoubleDamage) {
        final_damage = final_damage.saturating_mul(2);
    }
    if modifier_has(mods_target, ModifierKind::Intangible) && final_damage > 1 {
        final_damage = 1;
    }

    // Thorns: triggers per attack instance regardless of damage actually dealt
    if id_actor != id_target && modifier_has(mods_target, ModifierKind::Thorns) {
        let stacks = modifier_stacks(mods_target, ModifierKind::Thorns);
        effect_queue.push_front(Effect {
            kind: EffectKind::DamageDeal {
                amount: stacks as u16,
            },
            id_source: None,
            target: Target::Direct(Some(id_actor)),
        });
    }

    if final_damage > 0 {
        effect_queue.push_front(Effect {
            kind: EffectKind::DamageDeal {
                amount: final_damage,
            },
            id_source: Some(id_source),
            target: Target::Direct(Some(id_target)),
        });
    }
}
