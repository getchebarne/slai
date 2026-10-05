use crate::consts::ENERGY_CAP;
use crate::game::GameState;
use crate::types::Combat;
use crate::types::DeltaSign;

pub fn process_effect_energy_delta(state: &mut GameState, sign: DeltaSign, amount: u16) {
    assert!(
        state.combat.active,
        "process_effect_energy_delta outside the Combat frame"
    );
    let Combat { energy, .. } = &mut state.combat;
    match sign {
        DeltaSign::Gain => {
            energy.energy_current = energy.energy_current.saturating_add(amount).min(ENERGY_CAP);
        }
        DeltaSign::Loss => {
            energy.energy_current = energy.energy_current.saturating_sub(amount);
        }
    }
}
