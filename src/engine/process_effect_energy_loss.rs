use crate::engine::ProcessEffectResult;
use crate::state::Energy;

pub fn process_effect_energy_loss(energy: &mut Energy, amount: u8) -> ProcessEffectResult {
    energy.current = energy.current.saturating_sub(amount);
    ProcessEffectResult::Continue
}
