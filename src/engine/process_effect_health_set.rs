use rand::Rng;

use crate::effect::Amount;
use crate::game::GameState;

// Sets health directly, bypassing the damage/loss triggers HealthDelta carries
// (Asleep wake, PlatedArmor, ModeShift). Relative amounts resolve against max HP.
pub fn process_effect_health_set(id_target: Option<usize>, state: &mut GameState, amount: Amount) {
    let id_target = id_target.expect("HealthSet requires id_target");
    let health_max = state.entities[id_target].vitals.health_max;
    let value = match amount {
        Amount::Absolute(a) => a,
        // f32 mirrors the source's (int)(maxHP * fraction) float truncation
        Amount::Relative {
            numerator,
            denominator,
        } => (health_max as f32 * (numerator as f32 / denominator as f32)) as u16,
        Amount::RelativeRounded {
            numerator,
            denominator,
        } => (health_max as f32 * (numerator as f32 / denominator as f32) + 0.5) as u16,
        Amount::Range { min, max } => state.rng.random_range(min..=max),
    };
    state.entities[id_target].vitals.health = value.min(health_max);
}
