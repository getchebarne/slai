use crate::engine::ProcessEffectResult;
use crate::entity::Entity;
use crate::modifier::modifier_set_not_new;

pub fn process_effect_modifier_set_not_new(
    character: usize,
    entities: &mut [Entity],
    alive_monsters: &[usize],
) -> ProcessEffectResult {
    modifier_set_not_new(&mut entities[character].modifiers);
    for &mid in alive_monsters {
        modifier_set_not_new(&mut entities[mid].modifiers);
    }
    ProcessEffectResult::Continue { top: vec![], bot: vec![] }
}
