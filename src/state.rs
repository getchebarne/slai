// GameState and supporting structs.

use std::collections::VecDeque;

use rand::rngs::SmallRng;

use crate::cards::Card;
use crate::character::Character;
use crate::consts::{MAP_HEIGHT, MAP_WIDTH, MAX_MONSTERS};
use crate::effect::Effect;
use crate::monsters::Monster;
use crate::types::*;

// Vitals: physical combat state
#[derive(Debug, Clone, Copy)]
pub struct Vitals {
    pub health: u16,
    pub health_max: u16,
    pub block: u16,
}

// Entity: the universal unit of identity
#[derive(Debug, Clone)]
pub struct Entity {
    pub kind: EntityKind,
}

#[derive(Debug, Clone)]
pub enum EntityKind {
    Character(Character),
    Monster(Monster),
    Card(Card),
    MapNode(MapNode), // TODO: rename to `Room`
}

// Energy
#[derive(Debug, Clone, Copy)]
pub struct Energy {
    pub current: u8,
    pub max: u8,
}

// Map — pure data. Operations live as free functions in `crate::map`.
#[derive(Debug, Clone, Copy)]
pub struct MapNode {
    pub y: usize,
    pub x: usize,
    pub room_type: RoomType,
    pub edges: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    /// Fresh run — haven't picked a starting node yet.
    Start,
    /// On a map node at `(y, x)` where `y < MAP_HEIGHT`.
    Overworld { y: usize, x: usize },
    /// In the boss room. The boss room isn't part of the grid.
    BossRoom,
}

#[derive(Debug, Clone, Copy)]
pub struct Map {
    pub nodes: [[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    pub position: Position,
}

// GameState: the single source of truth
#[derive(Debug, Clone)]
pub struct GameState {
    pub ascension: u8,
    pub rng: SmallRng,

    // State-machine to track game phases, e.g.
    //     map
    //     combat (default or discard selection)
    //     rest site
    //     card reward selection
    pub phase: Phase,

    // Effect queue (the program)
    pub effect_queue: VecDeque<Effect>,

    // Map
    pub map: Map,

    // Energy
    pub energy: Energy,

    // Entities TODO: max entities?
    // --------
    pub entities: Vec<Entity>,

    // Entities / Character
    pub character: usize,

    // Entities / Monsters
    pub monsters: [usize; MAX_MONSTERS],
    pub monster_count: u8,

    // Entities / Monsters / Target
    pub card_target: Option<usize>,

    // Entities / Card / Deck
    pub deck: Vec<usize>,

    // Entities / Card / Combat piles
    pub draw_pile: Vec<usize>,
    pub hand: Vec<usize>,
    pub discard_pile: Vec<usize>,
    pub exhaust_pile: Vec<usize>,

    // Entities / Card / Combat piles
    pub card_rewards: Vec<usize>,
}
