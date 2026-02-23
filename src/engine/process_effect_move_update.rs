use rand::Rng;

use crate::engine::ProcessEffectResult;
use crate::monsters::{self, Monster};

pub fn process_effect_move_update(
    monster: &mut Monster,
    rng: &mut impl Rng,
) -> ProcessEffectResult {
    let move_next = monsters::get_next_move(monster, rng);
    monster.move_current = Some(move_next);
    monster.move_history.push(move_next);
    ProcessEffectResult::Pass
}
