use crate::types::Vitals;
use crate::types::Phase;

pub fn process_effect_max_health_gain(vitals: &mut Vitals, amount: u16) -> Option<Phase> {
    vitals.health_max = vitals.health_max.saturating_add(amount);
    None
}
