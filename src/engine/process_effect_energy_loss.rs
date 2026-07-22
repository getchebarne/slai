use crate::game::GameState;
use crate::types::Mode;

pub fn process_effect_energy_loss(state: &mut GameState, amount: u8) {
    let Mode::Combat { energy, .. } = &mut state.mode else {
        unreachable!("process_effect_energy_loss outside Combat mode")
    };
    energy.energy_current = energy.energy_current.saturating_sub(amount);
}
