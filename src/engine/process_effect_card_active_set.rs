use crate::engine::ProcessEffectResult;
use crate::types::EntityId;

pub fn process_effect_card_active_set(
    card_active: &mut Option<EntityId>,
    card_id: EntityId,
) -> ProcessEffectResult {
    *card_active = Some(card_id);
    ProcessEffectResult::Pass
}
