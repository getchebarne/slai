// GameState and supporting structs.

use std::collections::VecDeque;

use rand::rngs::SmallRng;

use crate::consts::{MAP_HEIGHT, MAP_WIDTH, MAX_MONSTERS};
use crate::effect::Effect;
use crate::entity::Entity;
use crate::types::*;

// Energy
#[derive(Debug, Clone, Copy)]
pub struct Energy {
    pub current: u8,
    pub max: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    // Fresh run — haven't picked a starting node yet
    Start,
    // On a map node at `(y, x)` where `y < MAP_HEIGHT`
    Overworld { y: usize, x: usize },
    // In the boss room. The boss room isn't part of the grid
    BossRoom,
}

#[derive(Debug, Clone, Copy)]
pub struct Map {
    pub id_nodes: [[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
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
    pub id_character: usize,

    // Entities / Monsters
    pub id_monsters: [usize; MAX_MONSTERS],
    pub monster_count: u8,

    // Entities / Monsters / Target
    pub id_card_target: Option<usize>,

    // Entities / Card / Deck
    pub id_deck: Vec<usize>,

    // Entities / Card / Combat piles
    pub id_draw_pile: Vec<usize>,
    pub id_hand: Vec<usize>,
    pub id_discard_pile: Vec<usize>,
    pub id_exhaust_pile: Vec<usize>,

    // Entities / Card / Combat rewards
    pub id_card_rewards: Vec<usize>,
}
