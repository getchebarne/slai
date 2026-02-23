use crate::engine::ProcessEffectResult;

pub fn process_effect_card_active_set(
    card_active: &mut Option<usize>,
    card_idx: usize,
) -> ProcessEffectResult {
    *card_active = Some(card_idx);
    ProcessEffectResult::Pass
}
