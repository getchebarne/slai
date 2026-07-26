use crate::consts::MODE_SHIFT_INCREASE_PER_CYCLE;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::Modifiers;
use crate::modifier::has_modifier;
use crate::modifier::modifier_apply;
use crate::modifier::modifier_def;
use crate::modifier::modifier_remove;
use crate::modifier::modifier_stacks;
use crate::types::RelicName;
use crate::utils::has_relic;

pub fn process_effect_modifier_gain(
    id_target: Option<usize>,
    state: &mut GameState,
    kind: ModifierKind,
    stacks: i16,
) {
    let id_target = id_target.expect("ModifierGain requires id_target");

    // Ginger / Turnip: negate the application outright, before Artifact is consumed
    if id_target == state.id_character
        && stacks > 0
        && ((kind == ModifierKind::Weak && has_relic(&state.id_relics, RelicName::Ginger))
            || (kind == ModifierKind::Frail && has_relic(&state.id_relics, RelicName::Turnip)))
    {
        return;
    }

    // Get mutable target reference
    let target = &mut state.entities[id_target];

    // Snecko Skull: +1 to any positive Poison application on a monster
    let stacks = if kind == ModifierKind::Poison
        && stacks > 0
        && matches!(target.kind, EntityKind::Monster)
        && has_relic(&state.id_relics, RelicName::SneckoSkull)
    {
        stacks + 1
    } else {
        stacks
    };

    // ModeShift has special scaling logic
    if kind == ModifierKind::ModeShift && target.kind == EntityKind::Monster {
        return process_mode_shift_gain(&mut target.modifiers, stacks, target.monster_cycle_count);
    }

    // Artifact
    let modifiers = &mut target.modifiers;
    let is_debuff_attempt = (stacks > 0 && !modifier_def(kind).is_buff)
        || (stacks < 0 && (kind == ModifierKind::Dexterity || kind == ModifierKind::Strength));

    if is_debuff_attempt && has_modifier(modifiers, ModifierKind::Artifact) {
        let stacks_new = modifier_stacks(modifiers, ModifierKind::Artifact) - 1;
        if stacks_new < modifier_def(ModifierKind::Artifact).stacks_min {
            modifier_remove(modifiers, ModifierKind::Artifact);
        } else {
            modifiers.stacks[ModifierKind::Artifact as usize] = stacks_new;
        }
        return;
    }

    modifier_apply(modifiers, kind, stacks);

    // Sadistic Nature: player-applied debuffs landing on a monster proc THORNS-type damage
    // Shackled is the buff-typed GainStrength half of Piercing Wail/Dark Shackles in StS
    if is_debuff_attempt
        && kind != ModifierKind::Shackled
        && state.entities[id_target].kind == EntityKind::Monster
    {
        let mods_char = &state.entities[state.id_character].modifiers;
        if has_modifier(mods_char, ModifierKind::SadisticNature) {
            let dmg = modifier_stacks(mods_char, ModifierKind::SadisticNature);
            state.effect_queue.push_front(Effect {
                kind: EffectKind::DamageDeal {
                    amount: dmg.max(0) as u16,
                },
                id_source: None,
                target: Target::Direct(Some(id_target)),
            });
        }
    }
}

fn process_mode_shift_gain(modifiers: &mut Modifiers, stacks: i16, monster_cycle_count: u8) {
    modifier_apply(
        modifiers,
        ModifierKind::ModeShift,
        stacks + MODE_SHIFT_INCREASE_PER_CYCLE * monster_cycle_count as i16,
    );
    modifiers.is_new[ModifierKind::ModeShift as usize] = false;
}
