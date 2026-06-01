use crate::game::GameState;

pub fn process_effect_energy_loss(state: &mut GameState, amount: u8) {
    state.energy.energy_current = state.energy.energy_current.saturating_sub(amount);
}
