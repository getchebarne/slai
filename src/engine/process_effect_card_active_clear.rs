use crate::engine::ProcessEffectResult;

pub fn process_effect_card_active_clear(
    card_active: &mut Option<usize>,
) -> ProcessEffectResult {
    *card_active = None;
    ProcessEffectResult::Pass
}
