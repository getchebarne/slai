use crate::engine::ProcessEffectResult;
use crate::modifier::modifier_set_not_new;
use crate::state::{Entity, EntityKind};

pub fn process_effect_modifier_set_not_new(
    character: usize,
    entities: &mut [Entity],
    alive_monsters: &[usize],
) -> ProcessEffectResult {
    let EntityKind::Character(c) = &mut entities[character].kind else {
        unreachable!()
    };
    modifier_set_not_new(&mut c.modifiers);

    for &mid in alive_monsters {
        let EntityKind::Monster(m) = &mut entities[mid].kind else {
            unreachable!()
        };
        modifier_set_not_new(&mut m.modifiers);
    }
    ProcessEffectResult::Continue
}
