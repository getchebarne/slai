use crate::engine::ProcessEffectResult;

pub fn process_effect_target_clear(card_target: &mut Option<usize>) -> ProcessEffectResult {
    *card_target = None;
    ProcessEffectResult::Continue
}
