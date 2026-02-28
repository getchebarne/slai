use crate::engine::ProcessEffectResult;
use crate::modifier::modifier_set_not_new;
use crate::state::{Entity, EntityKind};

pub fn process_effect_modifier_set_not_new(
    entities: &mut [Option<Entity>],
) -> ProcessEffectResult {
    for entity in entities.iter_mut().flatten() {
        match &mut entity.kind {
            EntityKind::Character(c) => modifier_set_not_new(&mut c.modifiers),
            EntityKind::Monster(m) => modifier_set_not_new(&mut m.modifiers),
            _ => {}
        }
    }
    ProcessEffectResult::Pass
}
