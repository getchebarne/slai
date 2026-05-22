use crate::game::GameState;

pub fn process_effect_max_health_loss(
    id_target: Option<usize>,
    state: &mut GameState,
    amount: u16,
) {
    let id_target = id_target.expect("MaxHealthLoss requires id_target");
    let vitals = &mut state.entities[id_target].vitals;
    vitals.health_max = vitals.health_max.saturating_sub(amount).max(1);
    vitals.health = vitals.health.min(vitals.health_max);
}
