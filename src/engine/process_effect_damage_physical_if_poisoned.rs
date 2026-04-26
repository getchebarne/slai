use std::collections::VecDeque;

use crate::effect::Effect;
use crate::engine::DispatchResult;
use crate::engine::process_effect_damage_physical::process_effect_damage_physical;
use crate::entity::Entity;
use crate::modifier::{ModifierKind, modifier_has};

// Bane's bonus hit: deal attack damage only if the target has Poison and is
// still alive. The dead-check prevents re-triggering Death on a corpse if
// the prior hit killed the target.
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
    let id_source_un = id_source.expect("DamagePhysicalIfPoisoned needs id_source");
    let source_mods = &entities[id_source_un].modifiers;
    process_effect_damage_physical(
        source_mods,
        &target.modifiers,
        id_source,
        id_target,
        amount,
        queue,
    )
}
