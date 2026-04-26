use std::collections::VecDeque;

use crate::consts::FACTOR_VULN;
use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::modifier::{ModifierKind, Modifiers, modifier_has};

// Non-attack damage from a power, on-death trigger, or per-play tick.
// Source-side modifiers (Strength/Weak) do NOT scale this; target Vulnerable
// still multiplies; block still subtracts. Pushes DamageDeal with no source
// so Envenom does not proc.
pub fn process_effect_damage_power(
    target_mods: &Modifiers,
    id_target: usize,
    amount: u16,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let mut value = amount as f32;
    if modifier_has(target_mods, ModifierKind::Vulnerable) {
        value *= FACTOR_VULN;
    }
    let final_damage = value.max(0.0) as u16;
    if final_damage > 0 {
        queue.push_front(Effect {
            kind: EffectKind::DamageDeal {
                amount: final_damage,
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
    }
    DispatchResult::Continue
}
