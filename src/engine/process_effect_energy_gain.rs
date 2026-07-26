use crate::game::GameState;
use crate::types::Mode;

pub fn process_effect_energy_gain(state: &mut GameState, amount: u16) {
    let Mode::Combat { energy, .. } = &mut state.mode else {
        unreachable!("process_effect_energy_gain outside Combat mode")
    };
    let next = (energy.energy_current as u16).saturating_add(amount);
    energy.energy_current = next.min(u8::MAX as u16) as u8;
}
