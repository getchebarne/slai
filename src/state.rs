// GameState and supporting structs.

use std::collections::VecDeque;

use rand::rngs::SmallRng;

use crate::cards::Card;
use crate::character::Character;
use crate::effect::Effect;
use crate::modifier::Modifiers;
use crate::monsters::Monster;
use crate::types::*;

// ---------------------------------------------------------------------------
// Vitals: physical combat state
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct Vitals {
    pub health: u16,
    pub health_max: u16,
    pub block: u16,
}

// ---------------------------------------------------------------------------
// Entity: the universal unit of identity
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Entity {
    pub kind: EntityKind,
}

#[derive(Debug, Clone)]
pub enum EntityKind {
    Character(Character),
    Monster(Monster),
    Card(Card),
}

impl EntityKind {
    pub fn combatant_mut(&mut self) -> (&mut Vitals, &mut Modifiers) {
        match self {
            EntityKind::Character(c) => (&mut c.vitals, &mut c.modifiers),
            EntityKind::Monster(m) => (&mut m.vitals, &mut m.modifiers),
            _ => panic!("Not a combatant"),
        }
    }

    pub fn combatant_ref(&self) -> (&Vitals, &Modifiers) {
        match self {
            EntityKind::Character(c) => (&c.vitals, &c.modifiers),
            EntityKind::Monster(m) => (&m.vitals, &m.modifiers),
            _ => panic!("Not a combatant"),
        }
    }

    pub fn character_ref(&self) -> &Character {
        match self {
            EntityKind::Character(c) => c,
            _ => panic!("Not a character"),
        }
    }

    pub fn character_mut(&mut self) -> &mut Character {
        match self {
            EntityKind::Character(c) => c,
            _ => panic!("Not a character"),
        }
    }

    pub fn monster_ref(&self) -> &Monster {
        match self {
            EntityKind::Monster(m) => m,
            _ => panic!("Not a monster"),
        }
    }

    pub fn monster_mut(&mut self) -> &mut Monster {
        match self {
            EntityKind::Monster(m) => m,
            _ => panic!("Not a monster"),
        }
    }

    pub fn card_ref(&self) -> &Card {
        match self {
            EntityKind::Card(card) => card,
            _ => panic!("Not a card"),
        }
    }
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
            return None;
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
    pub ascension: u8,
    pub fsm: Fsm,
    pub rng: SmallRng,

    pub entities: Vec<Option<Entity>>,

    pub energy: Energy,

    // Permanent deck (template — copied into entity array at combat start)
    pub deck: Vec<Card>,

    // Card piles (EntityIds referencing Card entities)
    pub draw_pile: Vec<EntityId>,
    pub hand: Vec<EntityId>,
    pub discard_pile: Vec<EntityId>,
    pub exhaust_pile: Vec<EntityId>,

    pub card_active: Option<EntityId>,
    pub card_target: Option<EntityId>,

    // Card rewards (templates, not entities)
    pub card_rewards: Vec<Card>,

    pub map: Map,

    pub effect_queue: VecDeque<Effect>,
}
