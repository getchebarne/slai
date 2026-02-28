use crate::engine::ProcessEffectResult;
use crate::types::EntityId;

pub fn process_effect_card_active_clear(
    card_active: &mut Option<EntityId>,
) -> ProcessEffectResult {
    *card_active = None;
    ProcessEffectResult::Pass
}
