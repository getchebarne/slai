use crate::modifier::Modifiers;
use crate::modifier::modifier_tick;

pub fn process_effect_modifier_tick(modifiers: &mut Modifiers) {
    modifier_tick(modifiers);
}
