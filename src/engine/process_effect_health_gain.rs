use crate::engine::DispatchResult;
use crate::types::Vitals;

pub fn process_effect_health_gain(vitals: &mut Vitals, amount: u16) -> DispatchResult {
    vitals.health = (vitals.health + amount).min(vitals.health_max);
    DispatchResult::Continue
}
