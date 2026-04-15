pub mod cultist;
pub mod fungi_beast;
pub mod jaw_worm;
pub mod the_guardian;

use crate::entity::{Entity, get_move_history_slice};
use crate::types::MonsterName;
use rand::Rng;

pub fn spawn_monster(
    monster_name: MonsterName,
    ascension_level: u8,
    rng: &mut impl Rng,
) -> Entity {
    match monster_name {
        MonsterName::Cultist => cultist::spawn_cultist(ascension_level, rng),
        MonsterName::JawWorm => jaw_worm::spawn_jaw_worm(ascension_level, rng),
        MonsterName::TheGuardian => the_guardian::spawn_the_guardian(ascension_level),
        MonsterName::FungiBeast => fungi_beast::spawn_fungi_beast(ascension_level, rng),
    }
}

pub fn get_next_move(entity: &Entity, rng: &mut impl Rng) -> usize {
    let history = get_move_history_slice(entity);
    match entity.monster_name {
        MonsterName::Cultist => cultist::get_next_move_cultist(entity.move_current, history),
        MonsterName::JawWorm => {
            jaw_worm::get_next_move_jaw_worm(entity.move_current, history, entity.moves, rng)
        }
        MonsterName::TheGuardian => the_guardian::get_next_move_the_guardian_full(
            entity.move_current,
            history,
            &entity.modifiers,
        ),
        MonsterName::FungiBeast => {
            fungi_beast::get_next_move_fungi_beast(entity.move_current, history, rng)
        }
    }
}
