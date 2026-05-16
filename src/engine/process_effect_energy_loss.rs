use crate::game::Energy;
use crate::types::Phase;

pub fn process_effect_energy_loss(energy: &mut Energy, amount: u8) -> Option<Phase> {
    energy.current = energy.current.saturating_sub(amount);
    None
}
