use crate::types::Phase;

pub fn process_effect_target_clear(card_target: &mut Option<usize>) -> Option<Phase> {
    *card_target = None;
    None
}
