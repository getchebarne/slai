use crate::game::GameState;
use crate::types::Mode;

pub fn process_effect_energy_loss(state: &mut GameState, amount: u8) {
    let Mode::Combat(combat) = &mut state.mode else {
        unreachable!("process_effect_energy_loss outside Combat mode")
    };
    combat.energy.energy_current = combat.energy.energy_current.saturating_sub(amount);
}
