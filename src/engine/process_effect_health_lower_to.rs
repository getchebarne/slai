use crate::effect::Amount;
use crate::effect::ReadAt;
use crate::effect::Rounding;
use crate::game::GameState;

// Lowers health to the amount, never raising it, bypassing the triggers HealthDelta carries
pub fn process_effect_health_lower_to(
    id_target: Option<usize>,
    state: &mut GameState,
    amount: Amount,
) {
    let id_target = id_target.expect("HealthLowerTo requires id_target");
    let health_max = state.entities[id_target].vitals.health_max;
    let value = match amount {
        Amount::Absolute(amount) => amount,

        // Float product, truncated once
        Amount::Relative {
            numerator,
            denominator,
            rounding: Rounding::Truncate,
            read_at: ReadAt::Now,
        } => (health_max as f32 * (numerator as f32 / denominator as f32)) as u16,
        _ => {
            unreachable!("HealthLowerTo only resolves Absolute or truncated Relative read now")
        }
    };
    let vitals = &mut state.entities[id_target].vitals;
    vitals.health = vitals.health.min(value);
}
