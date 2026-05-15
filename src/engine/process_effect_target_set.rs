use crate::types::Phase;

pub fn process_effect_target_set(
    card_target: &mut Option<usize>,
    id_target: usize,
) -> Option<Phase> {
    *card_target = Some(id_target);
    None
}
