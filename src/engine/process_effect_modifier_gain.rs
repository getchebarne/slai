use crate::consts::MODE_SHIFT_INCREASE_PER_CYCLE;
use crate::engine::DispatchResult;
use crate::modifier::{
    ModifierKind, Modifiers, modifier_apply, modifier_def, modifier_has, modifier_remove,
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

    // Negative stacks reduce existing modifier, removing if below minimum
    if stacks < 0 {
        if modifier_has(modifiers, kind) {
            let idx = kind as usize;
            modifiers.stacks[idx] += stacks;
            let cfg = modifier_def(kind);
            if modifiers.stacks[idx] < cfg.stacks_min {
                modifier_remove(modifiers, kind);
            }
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
    // ModeShift threshold increases each completed cycle.
    let increase = MODE_SHIFT_INCREASE_PER_CYCLE * cycle_count as i16;

    modifier_apply(modifiers, ModifierKind::ModeShift, stacks + increase);
    modifiers.is_new[ModifierKind::ModeShift as usize] = false;

    DispatchResult::Continue
}
