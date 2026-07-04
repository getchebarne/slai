use crate::effect::HealthDeltaAmount;
use crate::game::GameState;
use crate::types::DeltaSign;

pub fn process_effect_max_health_delta(
    id_target: Option<usize>,
    state: &mut GameState,
    sign: DeltaSign,
    amount: HealthDeltaAmount,
) {
    let id_target = id_target.expect("MaxHealthDelta requires id_target");
    let amount = match amount {
        HealthDeltaAmount::Absolute(a) => a,
        HealthDeltaAmount::Relative {
            numerator,
            denominator,
        }
        | HealthDeltaAmount::RelativeRounded {
            numerator,
            denominator,
        } => {
            let health_max = state.entities[id_target].vitals.health_max;
            // f32 mirrors the source's (int)(maxHP * fraction) float truncation
            let mut raw = health_max as f32 * (numerator as f32 / denominator as f32);
            if matches!(amount, HealthDeltaAmount::RelativeRounded { .. }) {
                raw += 0.5;
            }
            let raw = raw as u32;
            match sign {
                DeltaSign::Loss => raw.max(1) as u16,
                DeltaSign::Gain => raw as u16,
            }
        }
    };
    let vitals = &mut state.entities[id_target].vitals;
    match sign {
        DeltaSign::Gain => {
            vitals.health_max = vitals.health_max.saturating_add(amount);
        }
        DeltaSign::Loss => {
            vitals.health_max = vitals.health_max.saturating_sub(amount).max(1);
            vitals.health = vitals.health.min(vitals.health_max);
        }
    }
}
