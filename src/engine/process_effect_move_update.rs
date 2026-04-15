use rand::Rng;

use crate::engine::ProcessEffectResult;
use crate::entity::{Entity, push_move_history};
use crate::monsters;

pub fn process_effect_move_update(entity: &mut Entity, rng: &mut impl Rng) -> ProcessEffectResult {
    let move_next = monsters::get_next_move(entity, rng);
    entity.move_current = Some(move_next);
    let move_idx = move_next as u8;
    push_move_history(entity, move_idx);
    if monsters::is_cycle_boundary(entity.monster_name, move_idx) {
        entity.cycle_count += 1;
    }
    ProcessEffectResult::Continue { top: vec![], bot: vec![] }
}
