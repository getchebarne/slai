use crate::engine::ProcessEffectResult;
use crate::modifier::modifier_set_not_new;
use crate::monsters::Monster;
use crate::state::Character;

pub fn process_effect_modifier_set_not_new(
    character: &mut Character,
    monsters: &mut [Monster],
) -> ProcessEffectResult {
    modifier_set_not_new(&mut character.vitals.modifiers);
    for m in monsters.iter_mut() {
        modifier_set_not_new(&mut m.vitals.modifiers);
    }
    ProcessEffectResult::Pass
}
