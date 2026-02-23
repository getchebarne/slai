use crate::engine::ProcessEffectResult;
use crate::state::Vitals;

pub fn process_effect_block_set(
    vitals: &mut Vitals,
    amount: u16,
) -> ProcessEffectResult {
    vitals.block = amount;
    ProcessEffectResult::Pass
}
