// Entities: every kind of thing that lives in `GameState.entities`.
//
// Factory functions (`spawn_silent`, `spawn_monster`, `get_card`) and static
// data tables (card definitions, monster move tables) live in their own
// modules. This file is only the type definitions — the complete vocabulary
// of what an entity can be.

use crate::effect::Effect;
use crate::modifier::Modifiers;
use crate::types::{
    CardColor, CardKind, CardName, CardRarity, MonsterKind, MonsterName, RoomType, Vitals,
};

// ───────── Top-level entity ─────────

#[derive(Debug, Clone)]
pub struct Entity {
    pub kind: EntityKind,
}

#[derive(Debug, Clone)]
pub enum EntityKind {
    Character(Character),
    Monster(Monster),
    Card(Card),
    Room(Room),
}

// ───────── Character ─────────

#[derive(Debug, Clone, Copy)]
pub struct Character {
    pub name: &'static str,
    pub vitals: Vitals,
    pub modifiers: Modifiers,
    pub reward_roll_offset: i8,
}

// ───────── Monster ─────────

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
    pub effects: &'static [Effect],
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

// ───────── Card ─────────

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub name: CardName,
    pub kind: CardKind,
    pub color: CardColor,
    pub rarity: CardRarity,
    pub cost: u8,
    pub upgraded: bool,
    pub exhaust: bool,
    pub innate: bool,
    pub requires_target: bool,
    pub effects: &'static [Effect],
}

// ───────── Room ─────────

#[derive(Debug, Clone, Copy)]
pub struct Room {
    pub y: usize,
    pub x: usize,
    pub room_type: RoomType,
    pub edges: u8,
}
