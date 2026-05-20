use crate::modifier::ModifierKind;
use crate::modifier::Modifiers;
use crate::modifier::modifier_remove;

pub fn process_effect_modifier_remove(
    modifiers: &mut Modifiers,
    kind: ModifierKind,
) {
    modifier_remove(modifiers, kind);
}
