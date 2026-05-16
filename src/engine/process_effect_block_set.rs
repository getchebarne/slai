use crate::types::Phase;
use crate::types::Vitals;

pub fn process_effect_block_set(vitals: &mut Vitals, amount: u16) -> Option<Phase> {
    vitals.block = amount;

    // Continue
    None
}
