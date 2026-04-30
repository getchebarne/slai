use crate::consts::MODE_SHIFT_INCREASE_PER_CYCLE;
use crate::engine::DispatchResult;
use crate::modifier::{
    ModifierKind, Modifiers, modifier_apply, modifier_def, modifier_has, modifier_remove,
    modifier_stacks,
};

pub fn process_effect_modifier_gain(
    modifiers: &mut Modifiers,
    kind: ModifierKind,
    stacks: i16,
    cycle_count: Option<u8>,
) -> DispatchResult {
    // ModeShift has special scaling logic
    if kind == ModifierKind::ModeShift {
        if let Some(cc) = cycle_count {
            return process_mode_shift_gain(modifiers, stacks, cc);
        }
    }

    // Check artifact
    if stacks > 0 && !modifier_def(kind).is_buff && modifier_has(modifiers, ModifierKind::Artifact)
    {
        let stacks_new = modifier_stacks(modifiers, ModifierKind::Artifact) - 1;
        if stacks_new < modifier_def(ModifierKind::Artifact).stacks_min {
            modifier_remove(modifiers, ModifierKind::Artifact);
        } else {
            modifiers.stacks[ModifierKind::Artifact as usize] = stacks_new;
        }
        // Early return without applying debuff
        return DispatchResult::Continue;
    }

    // Negative stacks reduce existing modifier, removing if below minimum.
    // For modifiers whose stacks_min < 0 (Strength, Dexterity), absent →
    // create with a negative value via modifier_apply (Lagavulin Siphon Soul).
    if stacks < 0 {
        if modifier_has(modifiers, kind) {
            let idx = kind as usize;
            modifiers.stacks[idx] += stacks;
            let mod_def = modifier_def(kind);
            if modifiers.stacks[idx] < mod_def.stacks_min {
                modifier_remove(modifiers, kind);
            }
        } else if modifier_def(kind).stacks_min < 0 {
            modifier_apply(modifiers, kind, stacks);
        }
        return DispatchResult::Continue;
    }

    modifier_apply(modifiers, kind, stacks);
    DispatchResult::Continue
}

fn process_mode_shift_gain(
    modifiers: &mut Modifiers,
    stacks: i16,
    cycle_count: u8,
) -> DispatchResult {
    // ModeShift threshold increases each completed cycle
    let increase = MODE_SHIFT_INCREASE_PER_CYCLE * cycle_count as i16;

    modifier_apply(modifiers, ModifierKind::ModeShift, stacks + increase);
    modifiers.is_new[ModifierKind::ModeShift as usize] = false;

    DispatchResult::Continue
}
