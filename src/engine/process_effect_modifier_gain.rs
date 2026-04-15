use crate::consts::MODE_SHIFT_INCREASE_PER_CYCLE;
use crate::engine::ProcessEffectResult;
use crate::modifier::{
    ModifierKind, Modifiers, modifier_apply, modifier_def, modifier_has, modifier_remove,
};

pub fn process_effect_modifier_gain(
    modifiers: &mut Modifiers,
    kind: ModifierKind,
    stacks: i16,
    cycle_count: Option<u8>,
) -> ProcessEffectResult {
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
        return ProcessEffectResult::Continue { top: vec![], bot: vec![] };
    }

    modifier_apply(modifiers, kind, stacks);
    ProcessEffectResult::Continue { top: vec![], bot: vec![] }
}

fn process_mode_shift_gain(
    modifiers: &mut Modifiers,
    stacks: i16,
    cycle_count: u8,
) -> ProcessEffectResult {
    // ModeShift threshold increases each completed cycle.
    let increase = MODE_SHIFT_INCREASE_PER_CYCLE * cycle_count as i16;

    modifier_apply(modifiers, ModifierKind::ModeShift, stacks + increase);
    modifiers.is_new[ModifierKind::ModeShift as usize] = false;

    ProcessEffectResult::Continue { top: vec![], bot: vec![] }
}
