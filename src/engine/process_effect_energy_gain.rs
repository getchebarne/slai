use crate::game::Energy;
use crate::types::Phase;

pub fn process_effect_energy_gain(energy: &mut Energy, amount: u16) -> Option<Phase> {
    let next = (energy.current as u16).saturating_add(amount);
    energy.current = next.min(u8::MAX as u16) as u8;
    None
}
