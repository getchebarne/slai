use crate::types::Phase;

pub fn process_effect_set_cost_override(
    card_cost_override: &mut Option<u8>,
    amount: u8,
) -> Option<Phase> {
    *card_cost_override = Some(amount);
    None
}
