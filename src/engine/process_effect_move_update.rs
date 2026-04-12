use rand::Rng;

use crate::engine::ProcessEffectResult;
use crate::monsters;
use crate::state::{Entity, EntityKind};

pub fn process_effect_move_update(entity: &mut Entity, rng: &mut impl Rng) -> ProcessEffectResult {
    let EntityKind::Monster(m) = &mut entity.kind else { unreachable!() };
    let move_next = monsters::get_next_move(m, rng);
    m.move_current = Some(move_next);
    m.push_history(move_next as u8);
    ProcessEffectResult::Continue
}
