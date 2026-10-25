use crate::effect::Amount;
use crate::game::GameState;
use crate::utils::resolve_health_fraction;

// Sets health to the amount, bypassing the triggers HealthDelta carries
pub fn process_effect_health_set(id_target: Option<usize>, state: &mut GameState, amount: Amount) {
    let id_target = id_target.expect("HealthSet requires id_target");
    let vitals = &mut state.entities[id_target].vitals;
    vitals.health =
        resolve_health_fraction(vitals.health_max, state.event.health_max_at_open, amount);
}
