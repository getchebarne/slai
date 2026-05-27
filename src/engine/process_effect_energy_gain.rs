use crate::game::GameState;

pub fn process_effect_energy_gain(state: &mut GameState, amount: u16) {
    let next = (state.energy.energy_current as u16).saturating_add(amount);
    state.energy.energy_current = next.min(u8::MAX as u16) as u8;
}
