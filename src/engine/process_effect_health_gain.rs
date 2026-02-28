use crate::engine::ProcessEffectResult;
use crate::state::Vitals;

pub fn process_effect_health_gain(vitals: &mut Vitals, amount: u16) -> ProcessEffectResult {
    vitals.health = (vitals.health + amount).min(vitals.health_max);
    ProcessEffectResult::Pass
}
