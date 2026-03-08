use crate::consts::MODE_SHIFT_INCREASE_PER_CYCLE;
use crate::engine::ProcessEffectResult;
use crate::modifier::{
    ModifierKind, Modifiers, modifier_apply, modifier_def, modifier_has, modifier_remove,
};

// TODO: shared constant
const TWIN_SLAM_MOVE_IDX: u8 = 6;

pub fn process_effect_modifier_gain(
    modifiers: &mut Modifiers,
    kind: ModifierKind,
    stacks: i16,
    move_history: Option<&[u8]>,
) -> ProcessEffectResult {
    // ModeShift has special scaling logic
    if kind == ModifierKind::ModeShift {
        if let Some(history) = move_history {
            return process_mode_shift_gain(modifiers, stacks, history);
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
        return ProcessEffectResult::Continue;
    }

    modifier_apply(modifiers, kind, stacks);
    ProcessEffectResult::Continue
}

fn process_mode_shift_gain(
    modifiers: &mut Modifiers,
    stacks: i16,
    move_history: &[u8],
) -> ProcessEffectResult {
    // ModeShift threshold increases each cycle (counted by Twin Slam appearances)
    let cycle_count = move_history
        .iter()
        .filter(|&&m| m == TWIN_SLAM_MOVE_IDX)
        .count();
    let increase = MODE_SHIFT_INCREASE_PER_CYCLE * cycle_count as i16;

    modifier_apply(modifiers, ModifierKind::ModeShift, stacks + increase);
    modifiers.is_new[ModifierKind::ModeShift as usize] = false;

    ProcessEffectResult::Continue
}
