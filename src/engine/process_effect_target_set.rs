use crate::engine::DispatchResult;

pub fn process_effect_target_set(
    card_target: &mut Option<usize>,
    id_target: usize,
) -> DispatchResult {
    *card_target = Some(id_target);
    DispatchResult::Continue
}
