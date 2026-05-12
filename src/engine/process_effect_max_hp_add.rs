use crate::engine::DispatchResult;
use crate::types::Vitals;

pub fn process_effect_max_hp_add(vitals: &mut Vitals, amount: u16) -> DispatchResult {
    vitals.health_max = vitals.health_max.saturating_add(amount);
    DispatchResult::Continue
}
