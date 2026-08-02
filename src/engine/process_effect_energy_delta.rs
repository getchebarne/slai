use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::Mode;

pub fn process_effect_energy_delta(state: &mut GameState, sign: DeltaSign, amount: u16) {
    let Some(Mode::Combat { energy, .. }) = state.mode_stack.last_mut() else {
        unreachable!("process_effect_energy_delta outside Combat mode")
    };
    match sign {
        DeltaSign::Gain => {
            let next = (energy.energy_current as u16).saturating_add(amount);
            energy.energy_current = next.min(u8::MAX as u16) as u8;
        }
        DeltaSign::Loss => {
            energy.energy_current = energy
                .energy_current
                .saturating_sub(amount.min(u8::MAX as u16) as u8);
        }
    }
}
