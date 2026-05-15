use crate::types::Vitals;
use crate::types::Phase;

pub fn process_effect_health_gain(vitals: &mut Vitals, amount: u16) -> Option<Phase> {
    vitals.health = (vitals.health + amount).min(vitals.health_max);
    None
}
