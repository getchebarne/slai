use crate::engine::ProcessEffectResult;
use crate::modifier::modifier_set_not_new;
use crate::state::Entity;
use crate::types::EntityId;

pub fn process_effect_modifier_set_not_new(
    character: EntityId,
    entities: &mut [Entity],
    alive_monsters: &[EntityId],
) -> ProcessEffectResult {
    modifier_set_not_new(
        &mut entities[character.0 as usize]
            .kind
            .character_mut()
            .modifiers,
    );
    for &mid in alive_monsters {
        modifier_set_not_new(&mut entities[mid.0 as usize].kind.monster_mut().modifiers);
    }
    ProcessEffectResult::Continue
}
