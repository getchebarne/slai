use crate::engine::ProcessEffectResult;
use crate::modifier::{ModifierKind, Modifiers, modifier_remove};

pub fn process_effect_modifier_remove(
    modifiers: &mut Modifiers,
    kind: ModifierKind,
) -> ProcessEffectResult {
    modifier_remove(modifiers, kind);
    ProcessEffectResult::Pass
}
