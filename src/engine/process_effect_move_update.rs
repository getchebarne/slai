use rand::Rng;

use crate::engine::DispatchResult;
use crate::entity::{Entity, push_move_history};
use crate::monsters::{get_next_move, is_cycle_boundary};

pub fn process_effect_move_update(
    entity: &mut Entity,
    ascension_level: u8,
    rng: &mut impl Rng,
) -> DispatchResult {
    // Get next move
    let move_next = get_next_move(entity, ascension_level, rng);

    // Set current move
    entity.move_current = Some(move_next);

    // Update move history
    let move_idx = move_next as u8;
    push_move_history(entity, move_idx);

    // Update cycle count
    if is_cycle_boundary(entity.monster_name, move_idx) {
        entity.cycle_count += 1;
    }
    DispatchResult::Continue
}
