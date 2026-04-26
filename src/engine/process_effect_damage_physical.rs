use std::collections::VecDeque;

use crate::consts::{FACTOR_VULN, FACTOR_WEAK};
use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::modifier::{ModifierKind, Modifiers, modifier_has, modifier_stacks};

pub fn process_effect_damage_physical(
    source_mods: &Modifiers,
    target_mods: &Modifiers,
    id_source: Option<usize>,
    id_target: usize,
    amount: u16,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let mut value = amount as f32;

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

    // Intangible: cap any incoming damage at 1 per attack instance,
    // pre-block (block then subtracts as normal). Matches StS
    // IntangiblePlayerPower.atDamageFinalReceive.
    if modifier_has(target_mods, ModifierKind::Intangible) && value > 1.0 {
        value = 1.0;
    }

    // Thorns: triggers per attack instance regardless of damage actually
    // dealt. Pushed before DamageDeal so the queue resolves DamageDeal first
    // (target takes the hit), then the Thorns reflect lands on the attacker.
    if let Some(id_src) = id_source {
        if id_src != id_target && modifier_has(target_mods, ModifierKind::Thorns) {
            let thorns_stacks = modifier_stacks(target_mods, ModifierKind::Thorns);
            queue.push_front(Effect {
                kind: EffectKind::DamagePower {
                    amount: thorns_stacks as u16,
                },
                id_source: None,
                target: Target::Direct(Some(id_src)),
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
