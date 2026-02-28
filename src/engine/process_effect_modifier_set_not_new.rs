use crate::engine::ProcessEffectResult;
use crate::modifier::modifier_set_not_new;
use crate::state::Entity;
use crate::types::EntityId;

pub fn process_effect_modifier_set_not_new(
    entities: &mut [Entity],
    alive_monsters: &[EntityId],
) -> ProcessEffectResult {
    modifier_set_not_new(&mut entities[0].kind.character_mut().modifiers);
    for &mid in alive_monsters {
        modifier_set_not_new(&mut entities[mid.0 as usize].kind.monster_mut().modifiers);
    }
    ProcessEffectResult::Pass
}
