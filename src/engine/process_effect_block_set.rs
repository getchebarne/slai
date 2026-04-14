use crate::engine::ProcessEffectResult;
use crate::types::Vitals;

pub fn process_effect_block_set(vitals: &mut Vitals, amount: u16) -> ProcessEffectResult {
    vitals.block = amount;

    // Continue
    ProcessEffectResult::Continue
}
