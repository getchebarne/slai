use crate::engine::ProcessEffectResult;
use crate::types::EntityId;

pub fn process_effect_target_clear(card_target: &mut Option<EntityId>) -> ProcessEffectResult {
    *card_target = None;
    ProcessEffectResult::Continue
}
