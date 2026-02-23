use crate::engine::ProcessEffectResult;

pub fn process_effect_target_set(
    card_target: &mut Option<u8>,
    monster_idx: u8,
) -> ProcessEffectResult {
    *card_target = Some(monster_idx);
    ProcessEffectResult::Pass
}
