use crate::engine::ProcessEffectResult;
use crate::state::Energy;

pub fn process_effect_energy_gain(energy: &mut Energy, amount: u8) -> ProcessEffectResult {
    energy.current = (energy.current + amount).min(255);
    ProcessEffectResult::Continue
}
