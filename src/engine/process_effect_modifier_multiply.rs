use crate::engine::DispatchResult;
use crate::modifier::{ModifierKind, Modifiers, modifier_def, modifier_has};

// Multiply target's stacks of `kind` by `factor`. No-op if target doesn't
// have the modifier (matches StS DoublePoisonAction guard). Catalyst uses
// this with factor=2 (base) or 3 (upgraded) on Poison.
pub fn process_effect_modifier_multiply(
    modifiers: &mut Modifiers,
    kind: ModifierKind,
    factor: u8,
) -> DispatchResult {
    if !modifier_has(modifiers, kind) {
        return DispatchResult::Continue;
    }
    let cfg = modifier_def(kind);
    let cur = modifiers.stacks[kind as usize] as i32;
    let new = (cur * factor as i32).clamp(cfg.stacks_min as i32, cfg.stacks_max as i32);
    modifiers.stacks[kind as usize] = new as i16;
    DispatchResult::Continue
}
