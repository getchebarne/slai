use crate::engine::ProcessEffectResult;

pub fn process_effect_target_set(
    card_target: &mut Option<usize>,
    target: usize,
) -> ProcessEffectResult {
    *card_target = Some(target);
    ProcessEffectResult::Continue
}
