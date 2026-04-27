use std::collections::VecDeque;

use crate::effect::Effect;
use crate::engine::DispatchResult;
use crate::engine::process_effect_damage_physical::process_effect_damage_physical;
use crate::entity::Entity;
use crate::modifier::{ModifierKind, modifier_has};

pub fn process_effect_damage_physical_if_poisoned(
    entities: &[Entity],
    id_source: Option<usize>,
    id_target: usize,
    amount: u16,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let target = &entities[id_target];
    if target.dead || !modifier_has(&target.modifiers, ModifierKind::Poison) {
        return DispatchResult::Continue;
    }
    let mods_source = &entities[id_source.unwrap()].modifiers;
    process_effect_damage_physical(
        mods_source,
        &target.modifiers,
        id_source,
        id_target,
        amount,
        queue,
    )
}
