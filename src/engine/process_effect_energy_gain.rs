use crate::engine::DispatchResult;
use crate::state::Energy;

pub fn process_effect_energy_gain(energy: &mut Energy, amount: u8) -> DispatchResult {
    energy.current = energy.current.saturating_add(amount);
    DispatchResult::Continue
}
