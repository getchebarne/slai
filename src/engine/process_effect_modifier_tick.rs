use crate::modifier::Modifiers;
use crate::modifier::modifier_tick;
use crate::types::Phase;

pub fn process_effect_modifier_tick(modifiers: &mut Modifiers) -> Option<Phase> {
    modifier_tick(modifiers);
    None
}
