use crate::engine::DispatchResult;
use crate::modifier::{ModifierKind, Modifiers, modifier_def, modifier_has};

// Multiply target's stacks of `kind` by `factor`. No-op if target doesn't
// have the modifier
pub fn process_effect_modifier_multiply(
    modifiers: &mut Modifiers,
    kind: ModifierKind,
    factor: u8,
) -> DispatchResult {
    if !modifier_has(modifiers, kind) {
        return DispatchResult::Continue;
    }
    let mod_def = modifier_def(kind);
    let stacks_cur = modifiers.stacks[kind as usize] as i32;
    let stacks_new =
        (stacks_cur * factor as i32).clamp(mod_def.stacks_min as i32, mod_def.stacks_max as i32);
    modifiers.stacks[kind as usize] = stacks_new as i16;
    DispatchResult::Continue
}
