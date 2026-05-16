use crate::types::Phase;
use crate::types::Vitals;

pub fn process_effect_max_health_loss(vitals: &mut Vitals, amount: u16) -> Option<Phase> {
    vitals.health_max = vitals.health_max.saturating_sub(amount).max(1);
    vitals.health = vitals.health.min(vitals.health_max);
    None
}
