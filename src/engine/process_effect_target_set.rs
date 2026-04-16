use crate::engine::DispatchResult;

pub fn process_effect_target_set(
    card_target: &mut Option<usize>,
    target: usize,
) -> DispatchResult {
    *card_target = Some(target);
    DispatchResult::Continue
}
