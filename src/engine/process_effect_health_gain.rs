use crate::game::GameState;

pub fn process_effect_health_gain(id_target: Option<usize>, state: &mut GameState, amount: u16) {
    let id_target = id_target.expect("HealthGain requires id_target");
    let vitals = &mut state.entities[id_target].vitals;
    vitals.health = (vitals.health + amount).min(vitals.health_max);
}
