use crate::types::Vitals;

pub fn process_effect_max_health_gain(vitals: &mut Vitals, amount: u16) {
    vitals.health_max = vitals.health_max.saturating_add(amount);
}
