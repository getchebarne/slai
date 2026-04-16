use crate::engine::DispatchResult;
use crate::types::Vitals;

pub fn process_effect_block_set(vitals: &mut Vitals, amount: u16) -> DispatchResult {
    vitals.block = amount;

    // Continue
    DispatchResult::Continue
}
