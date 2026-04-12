use crate::engine::ProcessEffectResult;
use crate::modifier::modifier_set_not_new;
use crate::state::{Entity, EntityKind};
use crate::types::EntityId;

pub fn process_effect_modifier_set_not_new(
    character: EntityId,
    entities: &mut [Entity],
    alive_monsters: &[EntityId],
) -> ProcessEffectResult {
    let EntityKind::Character(c) = &mut entities[character.0 as usize].kind else {
        unreachable!()
    };
    modifier_set_not_new(&mut c.modifiers);

    for &mid in alive_monsters {
        let EntityKind::Monster(m) = &mut entities[mid.0 as usize].kind else {
            unreachable!()
        };
        modifier_set_not_new(&mut m.modifiers);
    }
    ProcessEffectResult::Continue
}
