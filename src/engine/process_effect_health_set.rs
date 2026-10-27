use crate::effect::Amount;
use crate::effect::EffectKind;
use crate::engine::first_target;
use crate::game::GameState;
use crate::utils::resolve_health_fraction;

// Sets health to the amount, bypassing the triggers HealthDelta carries
pub fn process_effect_health_set(
    id_source: Option<usize>,
    id_targets: &[usize],
    state: &mut GameState,
    amount: Amount,
) {
    let id_target = first_target(
        state,
        EffectKind::HealthSet { amount },
        id_source,
        id_targets,
    );
    let id_target = id_target.expect("HealthSet requires id_target");
    let vitals = &mut state.entities[id_target].vitals;
    vitals.health =
        resolve_health_fraction(vitals.health_max, state.event.health_max_at_open, amount);
}
