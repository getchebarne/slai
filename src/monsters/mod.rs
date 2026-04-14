pub mod cultist;
pub mod fungi_beast;
pub mod jaw_worm;
pub mod the_guardian;

use crate::entities::Monster;
use crate::types::MonsterName;
use rand::Rng;

pub fn spawn_monster(
    monster_name: MonsterName,
    ascension_level: u8,
    rng: &mut impl Rng,
) -> Monster {
    match monster_name {
        MonsterName::Cultist => cultist::spawn_cultist(ascension_level, rng),
        MonsterName::JawWorm => jaw_worm::spawn_jaw_worm(ascension_level, rng),
        MonsterName::TheGuardian => the_guardian::spawn_the_guardian(ascension_level),
        MonsterName::FungiBeast => fungi_beast::spawn_fungi_beast(ascension_level, rng),
    }
}

pub fn get_next_move(monster: &Monster, rng: &mut impl Rng) -> usize {
    let history = monster.history_slice();
    match monster.name {
        MonsterName::Cultist => cultist::get_next_move_cultist(monster.move_current, history),
        MonsterName::JawWorm => {
            jaw_worm::get_next_move_jaw_worm(monster.move_current, history, monster.moves, rng)
        }
        MonsterName::TheGuardian => the_guardian::get_next_move_the_guardian_full(
            monster.move_current,
            history,
            &monster.modifiers,
        ),
        MonsterName::FungiBeast => {
            fungi_beast::get_next_move_fungi_beast(monster.move_current, history, rng)
        }
    }
}
