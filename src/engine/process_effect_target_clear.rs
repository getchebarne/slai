use crate::engine::DispatchResult;

pub fn process_effect_target_clear(card_target: &mut Option<usize>) -> DispatchResult {
    *card_target = None;
    DispatchResult::Continue
}
