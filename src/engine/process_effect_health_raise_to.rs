use crate::effect::Amount;
use crate::game::GameState;
use crate::utils::resolve_health_fraction;

// Raises health to the amount, never lowering it
pub fn process_effect_health_raise_to(
    id_target: Option<usize>,
    state: &mut GameState,
    amount: Amount,
) {
    let id_target = id_target.expect("HealthRaiseTo requires id_target");
    let vitals = &mut state.entities[id_target].vitals;
    let amount = resolve_health_fraction(vitals.health_max, state.event.health_max_at_open, amount);
    vitals.health = vitals.health.max(amount);
}
