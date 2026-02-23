use crate::engine::ProcessEffectResult;
use crate::modifier::{Modifiers, modifier_tick};

pub fn process_effect_modifier_tick(
    modifiers: &mut Modifiers,
) -> ProcessEffectResult {
    modifier_tick(modifiers);
    ProcessEffectResult::Pass
}
