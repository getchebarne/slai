use crate::engine::DispatchResult;

pub fn process_effect_set_cost_override(
    card_cost_override: &mut Option<u8>,
    amount: u8,
) -> DispatchResult {
    *card_cost_override = Some(amount);
    DispatchResult::Continue
}
