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
    MapNode(MapNode),
}

// Energy
#[derive(Debug, Clone, Copy)]
pub struct Energy {
    pub current: u8,
    pub max: u8,
}

// Map
#[derive(Debug, Clone, Copy)]
pub struct MapNode {
    pub y: usize,
    pub x: usize,
    pub room_type: RoomType,
    pub edges: u8,
}

impl MapNode {
    pub fn has_edge(&self, x: usize) -> bool {
        self.edges & (1 << x) != 0
    }

    pub fn edge_indices(&self) -> impl Iterator<Item = usize> {
        let edges = self.edges;
        (0..MAP_WIDTH).filter(move |&x| edges & (1 << x) != 0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Map {
    pub nodes: [[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    pub y_current: Option<usize>,
    pub x_current: Option<usize>,
}

impl Map {
    pub fn node_at<'a>(&self, entities: &'a [Entity], y: usize, x: usize) -> Option<&'a MapNode> {
        let id = self.nodes[y][x]?;
        let EntityKind::MapNode(node) = &entities[id].kind else {
            unreachable!()
        };
        Some(node)
    }

    pub fn active_node<'a>(&self, entities: &'a [Entity]) -> Option<&'a MapNode> {
        let y = self.y_current?;
        let x = self.x_current?;
        if y >= MAP_HEIGHT {
            return None;
        }
        self.node_at(entities, y, x)
    }

    pub fn active_room_type(&self, entities: &[Entity]) -> Option<RoomType> {
        let y = self.y_current?;
        if y == MAP_HEIGHT {
            return Some(RoomType::CombatBoss);
        }
        self.active_node(entities).map(|n| n.room_type)
    }

    pub fn is_boss_room(&self) -> bool {
        self.y_current == Some(MAP_HEIGHT)
    }
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
