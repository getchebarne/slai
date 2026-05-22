use crate::game::GameState;

pub fn process_effect_energy_loss(state: &mut GameState, amount: u8) {
    state.energy.current = state.energy.current.saturating_sub(amount);
}
