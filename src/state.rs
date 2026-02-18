// GameState and supporting structs.

use std::collections::VecDeque;

use rand::rngs::SmallRng;

use crate::effect::Effect;
use crate::modifier::{Modifiers, modifiers_new};
use crate::types::*;

// ---------------------------------------------------------------------------
// Card: immutable card blueprint
// ---------------------------------------------------------------------------

use crate::effect::EffectTemplate;

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub name: CardName,
    pub type_: CardType,
    pub color: CardColor,
    pub rarity: CardRarity,
    pub cost: u8,
    pub upgraded: bool,
    pub exhaust: bool,
    pub innate: bool,
    pub effects: &'static [EffectTemplate],
}

impl Card {
    pub fn requires_target(&self) -> bool {
        use crate::effect::TargetKind;
        self.effects.iter().any(|e| {
            matches!(
                e,
                EffectTemplate::DamagePhysical {
                    target: TargetKind::CardTarget,
                    ..
                } | EffectTemplate::BlockGain {
                    target: TargetKind::CardTarget,
                    ..
                } | EffectTemplate::ModifierGain {
                    target: TargetKind::CardTarget,
                    ..
                } | EffectTemplate::ModifierRemove {
                    target: TargetKind::CardTarget,
                    ..
                }
            )
        })
    }

    pub fn requires_discard(&self) -> bool {
        use crate::effect::SelectionKind;
        self.effects.iter().any(|e| {
            matches!(
                e,
                EffectTemplate::CardDiscard {
                    selection: SelectionKind::Input
                }
            )
        })
    }
}

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
// Intent (for monsters)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default)]
pub struct Intent {
    pub damage: Option<u16>,
    pub instances: Option<u8>,
    pub block: bool,
    pub buff: bool,
    pub debuff: bool,
}

impl Intent {
    pub const fn attack(damage: u16, instances: u8) -> Self {
        Intent {
            damage: Some(damage),
            instances: Some(instances),
            block: false,
            buff: false,
            debuff: false,
        }
    }

    pub const fn attack_block(damage: u16, instances: u8) -> Self {
        Intent {
            damage: Some(damage),
            instances: Some(instances),
            block: true,
            buff: false,
            debuff: false,
        }
    }

    pub const fn buff() -> Self {
        Intent {
            damage: None,
            instances: None,
            block: false,
            buff: true,
            debuff: false,
        }
    }

    pub const fn buff_block() -> Self {
        Intent {
            damage: None,
            instances: None,
            block: true,
            buff: true,
            debuff: false,
        }
    }

    pub const fn block_only() -> Self {
        Intent {
            damage: None,
            instances: None,
            block: true,
            buff: false,
            debuff: false,
        }
    }

    pub const fn debuff() -> Self {
        Intent {
            damage: None,
            instances: None,
            block: false,
            buff: false,
            debuff: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Monster Move
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Move {
    pub name: &'static str,
    pub effects: &'static [EffectTemplate],
    pub intent: Intent,
}

// ---------------------------------------------------------------------------
// Monster
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Monster {
    pub name: MonsterName,
    pub kind: MonsterKind,
    pub vitals: Vitals,
    pub moves: Vec<Move>,
    pub move_current: Option<usize>,
    pub move_history: Vec<usize>,
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
