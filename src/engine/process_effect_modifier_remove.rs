use crate::engine::DispatchResult;
use crate::modifier::{ModifierKind, Modifiers, modifier_remove};

pub fn process_effect_modifier_remove(
    modifiers: &mut Modifiers,
    kind: ModifierKind,
) -> DispatchResult {
    modifier_remove(modifiers, kind);
    DispatchResult::Continue
}
