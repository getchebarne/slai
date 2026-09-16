use crate::game::GameState;

// VampireDamageAction heals for lastDamageTaken, i.e. the HP the target really lost
pub fn process_effect_lifesteal_heal(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("LifestealHeal requires id_target");
    let heal = state.combat.last_health_lost;
    let vitals = &mut state.entities[id_target].vitals;
    vitals.health = (vitals.health + heal).min(vitals.health_max);
}
