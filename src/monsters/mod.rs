pub mod cultist;
pub mod fungi_beast;
pub mod jaw_worm;
pub mod the_guardian;

use crate::effect::EffectTemplate;
use crate::state::Vitals;
use crate::types::EntityId;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub enum Intent {
    Attack { damage: u16, instances: u8 },
    AttackBlock { damage: u16, instances: u8 },
    AttackBuff { damage: u16, instances: u8 },
    Block,
    BlockBuff,
    Buff,
    Debuff,
    DebuffPowerful,
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub name: &'static str,
    pub effects: &'static [EffectTemplate],
    pub intent: Intent,
}

#[derive(Debug, Clone)]
pub struct Monster {
    pub id: EntityId,
    pub name: MonsterName,
    pub kind: MonsterKind,
    pub vitals: Vitals,
    pub moves: &'static [Move],
    pub move_current: Option<usize>,
    pub move_history: Vec<usize>,
}

pub fn spawn_monster(
    id: EntityId,
    monster_name: MonsterName,
    ascension_level: u8,
    rng: &mut impl Rng,
) -> Monster {
    match monster_name {
        MonsterName::Cultist => cultist::spawn_cultist(id, ascension_level, rng),
        MonsterName::JawWorm => jaw_worm::spawn_jaw_worm(id, ascension_level, rng),
        MonsterName::TheGuardian => the_guardian::spawn_the_guardian(id, ascension_level),
        MonsterName::FungiBeast => fungi_beast::spawn_fungi_beast(id, ascension_level, rng),
    }
}

pub fn get_next_move(monster: &Monster, rng: &mut impl Rng) -> usize {
    match monster.name {
        MonsterName::Cultist => cultist::get_next_move_cultist(monster),
        MonsterName::JawWorm => jaw_worm::get_next_move_jaw_worm(monster, rng),
        MonsterName::TheGuardian => the_guardian::get_next_move_the_guardian(monster),
        MonsterName::FungiBeast => fungi_beast::get_next_move_fungi_beast(monster, rng),
    }
}
