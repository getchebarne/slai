use crate::effect::HealthDeltaAmount;
use crate::effect::HealthDeltaSign;
use crate::game::GameState;

pub fn process_effect_max_health_delta(
    id_target: Option<usize>,
    state: &mut GameState,
    sign: HealthDeltaSign,
    amount: HealthDeltaAmount,
) {
    let id_target = id_target.expect("MaxHealthDelta requires id_target");
    let amount = match amount {
        HealthDeltaAmount::Absolute(a) => a,
        HealthDeltaAmount::Relative {
            numerator,
            denominator,
        } => {
            let health_max = state.entities[id_target].vitals.health_max;
            let raw = (health_max as u32 * numerator as u32) / denominator as u32;
            match sign {
                HealthDeltaSign::Loss => raw.max(1) as u16,
                HealthDeltaSign::Gain => raw as u16,
            }
        }
    };
    let vitals = &mut state.entities[id_target].vitals;
    match sign {
        HealthDeltaSign::Gain => {
            vitals.health_max = vitals.health_max.saturating_add(amount);
        }
        HealthDeltaSign::Loss => {
            vitals.health_max = vitals.health_max.saturating_sub(amount).max(1);
            vitals.health = vitals.health.min(vitals.health_max);
        }
    }
}
