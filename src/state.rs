// GameState and supporting structs.

use std::collections::VecDeque;

use rand::rngs::SmallRng;

use crate::cards::Card;
use crate::effect::Effect;
use crate::modifier::{Modifiers, modifiers_new};
use crate::monsters::Monster;
use crate::types::*;

// ---------------------------------------------------------------------------
// Vitals: shared health/block/modifier state
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Vitals {
    pub health: u16,
    pub health_max: u16,
    pub block: u16,
    pub modifiers: Modifiers,
}

pub fn vitals_new(health: u16, health_max: u16) -> Vitals {
    Vitals {
        health,
        health_max,
        block: 0,
        modifiers: modifiers_new(),
    }
}

// ---------------------------------------------------------------------------
// Character
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Character {
    pub vitals: Vitals,
    pub reward_roll_offset: i8,
}

// ---------------------------------------------------------------------------
// Energy
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct Energy {
    pub current: u8,
    pub max: u8,
}

// ---------------------------------------------------------------------------
// Map
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct MapNode {
    pub y: usize,
    pub x: usize,
    pub room_type: RoomType,
    pub x_next: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Map {
    pub nodes: Vec<Vec<Option<MapNode>>>,
    pub active_y: Option<usize>,
    pub active_x: Option<usize>,
    pub boss_room_y: usize,
}

impl Map {
    pub fn active_node(&self) -> Option<&MapNode> {
        let y = self.active_y?;
        let x = self.active_x?;
        if y >= self.nodes.len() {
            return None; // boss room is virtual, not in the grid
        }
        self.nodes[y][x].as_ref()
    }

    pub fn active_room_type(&self) -> Option<RoomType> {
        let y = self.active_y?;
        if y == self.boss_room_y {
            return Some(RoomType::CombatBoss);
        }
        self.active_node().map(|n| n.room_type)
    }

    pub fn is_boss_room(&self) -> bool {
        self.active_y == Some(self.boss_room_y)
    }
}

// ---------------------------------------------------------------------------
// GameState: the single source of truth
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct GameState {
    // Meta
    pub ascension: u8,
    pub fsm: Fsm,
    pub rng: SmallRng,

    // Character
    pub character: Character,
    pub energy: Energy,

    // Permanent deck
    pub deck: Vec<Card>,

    // Combat card pool + piles (indices into combat_cards)
    pub combat_cards: Vec<Card>,
    pub draw_pile: Vec<usize>,
    pub hand: Vec<usize>,
    pub discard_pile: Vec<usize>,
    pub exhaust_pile: Vec<usize>,

    // Active card / target
    pub card_active: Option<usize>,
    pub card_target: Option<u8>,

    // Monsters
    pub monsters: Vec<Monster>,

    // Card rewards
    pub card_rewards: Vec<Card>,

    // Map
    pub map: Map,

    // Effect queue
    pub effect_queue: VecDeque<Effect>,
}
