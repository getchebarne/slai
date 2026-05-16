use rand::Rng;

use crate::entity::Entity;
use crate::entity::push_move_history;
use crate::monsters::get_next_move;
use crate::monsters::is_cycle_boundary;
use crate::types::Phase;

pub fn process_effect_move_update(
    entity: &mut Entity,
    entity_id: usize,
    id_alive_monsters: &[usize],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> Option<Phase> {
    // Get next move
    let move_next = get_next_move(entity, entity_id, id_alive_monsters, ascension_level, rng);

    // Set current move
    entity.move_current = Some(move_next);

    // Update move history
    let move_idx = move_next as u8;
    push_move_history(entity, move_idx);

    // Update cycle count
    if is_cycle_boundary(entity.monster_name, move_idx) {
        entity.monster_cycle_count += 1;
    }
    None
}
