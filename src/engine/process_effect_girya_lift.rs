use crate::game::GameState;
use crate::types::RelicName;

// Girya: each lift adds one combat-start Strength stack (the cap is enforced at the action layer)
pub fn process_effect_girya_lift(state: &mut GameState) {
    let id = state.id_relics[RelicName::Girya as usize].expect("GiryaLift requires Girya");
    state.entities[id].relic_counter += 1;
}
