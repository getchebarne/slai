use crate::engine::DispatchResult;
use crate::modifier::{Modifiers, modifier_tick};

pub fn process_effect_modifier_tick(modifiers: &mut Modifiers) -> DispatchResult {
    modifier_tick(modifiers);
    DispatchResult::Continue
}
