use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;
use crate::utils::get_id_actor;

// One attack hit whose damage is already final: the cancels, Bane's gate, Thorns reflect
pub fn process_effect_damage_physical(
    id_source: Option<usize>,
    id_target: Option<usize>,
    state: &mut GameState,
    amount: u16,
    if_poisoned: bool, // Bane
    lifesteal: bool,   // Life Suck
) {
    let id_source = id_source.expect("DamagePhysical requires id_source");
    let id_target = id_target.expect("DamagePhysical requires id_target");

    // A target killed by an earlier hit takes nothing more
    let target = &state.entities[id_target];
    if target.dead {
        return;
    }

    // Bane: the second hit only lands if the target is still Poisoned
    if if_poisoned && !has_modifier(&target.modifiers, ModifierKind::Poison) {
        return;
    }

    // A dying attacker's remaining hits are cancelled
    let id_actor = get_id_actor(&state.entities, state.id_character, id_source);
    if state.entities[id_actor].dead {
        return;
    }

    // Executes in reverse:
    //     1. DamageDeal (attack)
    //     2. DamageDeal (Thorns reflect)
    // Thorns: triggers per attack instance regardless of damage actually dealt
    let mods_target = &state.entities[id_target].modifiers;
    if id_actor != id_target && has_modifier(mods_target, ModifierKind::Thorns) {
        let stacks = modifier_stacks(mods_target, ModifierKind::Thorns);
        state.effect_queue.push_front(Effect {
            kind: EffectKind::DamageDeal {
                amount: stacks as u16,
                lifesteal: false,
            },
            id_source: None,
            target: Target::Direct(Some(id_actor)),
        });
    }

    if amount > 0 {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::DamageDeal { amount, lifesteal },
            id_source: Some(id_source),
            target: Target::Direct(Some(id_target)),
        });
    }
}
