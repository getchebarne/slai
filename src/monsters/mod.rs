pub mod cultist;
pub mod fungi_beast;
pub mod jaw_worm;
pub mod the_guardian;

use crate::effect::EffectTemplate;
use crate::modifier::Modifiers;
use crate::state::Vitals;
use crate::types::{MonsterKind, MonsterName};
use rand::Rng;

pub const MAX_MOVE_HISTORY: usize = 64;

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

#[derive(Debug, Clone, Copy)]
pub struct Monster {
    pub name: MonsterName,
    pub monster_kind: MonsterKind,
    pub vitals: Vitals,
    pub modifiers: Modifiers,
    pub moves: &'static [Move],
    pub move_current: Option<usize>,
    pub move_history: [u8; MAX_MOVE_HISTORY],
    pub move_history_len: u8,
    pub dead: bool,
}

impl Monster {
    pub fn push_history(&mut self, move_idx: u8) {
        assert!(
            (self.move_history_len as usize) < MAX_MOVE_HISTORY,
            "move_history overflow"
        );
        self.move_history[self.move_history_len as usize] = move_idx;
        self.move_history_len += 1;
    }

    pub fn history_slice(&self) -> &[u8] {
        &self.move_history[..self.move_history_len as usize]
    }
}

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
