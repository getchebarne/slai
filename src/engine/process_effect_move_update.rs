use rand::Rng;

use crate::engine::DispatchResult;
use crate::entity::{Entity, push_move_history};
use crate::monsters;

pub fn process_effect_move_update(
    entity: &mut Entity,
    ascension_level: u8,
    rng: &mut impl Rng,
) -> DispatchResult {
    let move_next = monsters::get_next_move(entity, ascension_level, rng);
    entity.move_current = Some(move_next);
    let move_idx = move_next as u8;
    push_move_history(entity, move_idx);
    if monsters::is_cycle_boundary(entity.monster_name, move_idx) {
        entity.cycle_count += 1;
    }
    DispatchResult::Continue
}
