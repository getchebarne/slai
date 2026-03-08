use crate::engine::ProcessEffectResult;
use crate::types::EntityId;

pub fn process_effect_target_set(
    card_target: &mut Option<EntityId>,
    target: EntityId,
) -> ProcessEffectResult {
    *card_target = Some(target);
    ProcessEffectResult::Continue
}
