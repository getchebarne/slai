use rand::rngs::SmallRng;
use std::collections::VecDeque;

use crate::consts::{MAP_HEIGHT, MAP_WIDTH, MAX_MONSTERS};
use crate::effect::Effect;
use crate::entity::Entity;
use crate::types::*;

#[derive(Debug, Clone, Copy)]
pub struct Energy {
    pub current: u8,
    pub max: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    Start,
    Overworld { y: usize, x: usize },
    BossRoom,
}

// GameState: the single source of truth
#[derive(Debug, Clone)]
pub struct GameState {
    pub ascension: u8,
    pub rng: SmallRng,

    pub phase: Phase,

    // Effect queue
    pub effect_queue: VecDeque<Effect>,

    // Map
    pub id_rooms: [[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    pub location: Location,

    // Energy
    pub energy: Energy,

    // Entities
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
