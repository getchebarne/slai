// GameState and supporting structs.

use std::collections::VecDeque;

use rand::rngs::SmallRng;

use crate::cards::Card;
use crate::character::Character;
use crate::consts::{MAP_HEIGHT, MAP_WIDTH, MAX_MONSTERS};
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

    pub fn card_mut(&mut self) -> &mut Card {
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

#[derive(Debug, Clone, Copy)]
pub struct MapNode {
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
    pub nodes: [[Option<MapNode>; MAP_WIDTH]; MAP_HEIGHT],
    pub y_current: Option<usize>,
    pub x_current: Option<usize>,
}

impl Map {
    pub fn active_node(&self) -> Option<&MapNode> {
        let y = self.y_current?;
        let x = self.x_current?;
        if y >= MAP_HEIGHT {
            return None;
        }
        self.nodes[y][x].as_ref()
    }

    pub fn active_room_type(&self) -> Option<RoomType> {
        let y = self.y_current?;
        if y == MAP_HEIGHT {
            return Some(RoomType::CombatBoss);
        }
        self.active_node().map(|n| n.room_type)
    }

    pub fn is_boss_room(&self) -> bool {
        self.y_current == Some(MAP_HEIGHT)
    }
}

// ---------------------------------------------------------------------------
// GameState: the single source of truth
// ---------------------------------------------------------------------------

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

    // Entities
    // --------
    pub entities: Vec<Entity>,

    // Entities / Character
    pub character: EntityId,

    // Entities / Monsters
    pub monsters: [EntityId; MAX_MONSTERS],
    pub monster_count: u8,

    // Entities / Monsters / Target
    pub card_target: Option<EntityId>,

    // Entities / Card / Deck
    pub deck: Vec<EntityId>,

    // Entities / Card / Combat piles
    pub draw_pile: Vec<EntityId>,
    pub hand: Vec<EntityId>,
    pub discard_pile: Vec<EntityId>,
    pub exhaust_pile: Vec<EntityId>,

    // Entities / Card / Combat piles
    pub card_rewards: Vec<EntityId>,
}
