use crate::engine::DispatchResult;
use crate::game::Energy;

pub fn process_effect_energy_loss(energy: &mut Energy, amount: u8) -> DispatchResult {
    energy.current = energy.current.saturating_sub(amount);
    DispatchResult::Continue
}
