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
    pub effect_queue: VecDeque<Effect>,
    pub location: Location,
    pub energy: Energy,

    pub entities: Vec<Entity>,
    pub id_rooms: [[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    pub id_character: usize,

    pub id_monsters: [usize; MAX_MONSTERS],
    pub monster_count: u8,
    pub id_card_target: Option<usize>,

    pub id_deck: Vec<usize>,
    pub id_pile_draw: Vec<usize>,
    pub id_hand: Vec<usize>,
    pub id_pile_discard: Vec<usize>,
    pub id_pile_exhaust: Vec<usize>,
    pub id_card_rewards: Vec<usize>,

    // Read by self-referential effects
    pub card_last_drawn: Option<usize>,
    pub card_last_played: Option<usize>,

    // Per-turn counters; reset in process_effect_turn_end_character
    pub cards_discarded_this_turn: u8,
    pub attacks_played_this_turn: u8,

    // Per-combat counter; reset at combat_start
    pub instances_of_damage_taken_this_combat: u8,

    // Nightmare-pending template snapshot id; flushed at next TurnStart
    pub id_card_nightmare: Option<usize>,
}
