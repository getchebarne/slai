use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::ScaledAt;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;
use crate::utils::scale_attack_damage;
use crate::utils::vuln_factor;
use crate::utils::weak_factor;

// Fixes one of Skewer's hits as its play starts, from the Character and the target as they stand; the hit waits behind everything queued
pub fn process_effect_skewer_proc(
    id_source: Option<usize>,
    id_target: Option<usize>,
    state: &mut GameState,
    damage: u16,
) {
    let id_target = id_target.expect("SkewerProc requires id_target");

    let char_modifiers = &state.entities[state.id_character].modifiers;
    let mods_target = &state.entities[id_target].modifiers;
    let vigor = if has_modifier(char_modifiers, ModifierKind::Vigor) {
        modifier_stacks(char_modifiers, ModifierKind::Vigor)
    } else {
        0
    };
    let str_stacks = if has_modifier(char_modifiers, ModifierKind::Strength) {
        modifier_stacks(char_modifiers, ModifierKind::Strength)
    } else {
        0
    };
    let amount = scale_attack_damage(
        (damage as i16 + vigor).max(0) as u16,
        str_stacks,
        has_modifier(char_modifiers, ModifierKind::DoubleDamage),
        has_modifier(char_modifiers, ModifierKind::PenNib),
        weak_factor(has_modifier(char_modifiers, ModifierKind::Weak), false),
        vuln_factor(has_modifier(mods_target, ModifierKind::Vulnerable), false),
        has_modifier(mods_target, ModifierKind::Flight),
    );
    state.effect_queue.push_back(Effect {
        kind: EffectKind::DamagePhysical {
            amount,
            instances: 1,
            lifesteal: false,
            if_poisoned: false,
            scaled_at: ScaledAt::PlayStart,
        },
        id_source,
        target: Target::Direct(Some(id_target)),
    });
}
