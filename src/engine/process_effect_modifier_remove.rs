use crate::modifier::ModifierKind;
use crate::modifier::Modifiers;
use crate::modifier::modifier_remove;
use crate::types::Phase;

pub fn process_effect_modifier_remove(
    modifiers: &mut Modifiers,
    kind: ModifierKind,
) -> Option<Phase> {
    modifier_remove(modifiers, kind);
    None
}
