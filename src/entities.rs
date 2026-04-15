// Entities: every kind of thing that lives in `GameState.entities`.
//
// One flat "fat" Entity struct holds all fields from all kinds. A runtime
// `EntityType` tag distinguishes them. The per-kind structs (`Character`,
// `Monster`, `Card`, `Room`) exist only as initialization types returned by
// the spawn/factory functions; conversion into `Entity` goes through the
// `make_entity_from_*` free functions below.

use crate::effect::Effect;
use crate::modifier::{Modifiers, modifiers_new};
use crate::types::{
    CardColor, CardKind, CardName, CardRarity, MonsterKind, MonsterName, RoomType, Vitals,
};

// ───────── EntityType tag ─────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityType {
    Character,
    Monster,
    Card,
    Room,
}

// ───────── Fat Entity struct ─────────

pub const MAX_MOVE_HISTORY: usize = 64;

#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub kind: EntityType,

    // Combatant (Character + Monster)
    pub vitals: Vitals,
    pub modifiers: Modifiers,

    // Character-only
    pub character_name: &'static str,
    pub reward_roll_offset: i8,

    // Monster-only
    pub monster_name: MonsterName,
    pub monster_kind: MonsterKind,
    pub moves: &'static [Move],
    pub move_current: Option<usize>,
    pub move_history: [u8; MAX_MOVE_HISTORY],
    pub move_history_len: u8,
    pub dead: bool,

    // Card-only
    pub card_name: CardName,
    pub card_kind: CardKind,
    pub card_color: CardColor,
    pub card_rarity: CardRarity,
    pub card_cost: u8,
    pub card_upgraded: bool,
    pub card_exhaust: bool,
    pub card_innate: bool,
    pub card_requires_target: bool,
    pub card_effects: &'static [Effect],

    // Room-only
    pub node_y: usize,
    pub node_x: usize,
    pub room_type: RoomType,
    pub edges: u8,
}

// ───────── Intent / Move (Monster-facing) ─────────

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

// ───────── Legacy init-types (returned by spawn/factory fns) ─────────

#[derive(Debug, Clone, Copy)]
pub struct Character {
    pub name: &'static str,
    pub vitals: Vitals,
    pub modifiers: Modifiers,
    pub reward_roll_offset: i8,
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

#[derive(Debug, Clone, Copy)]
pub struct Room {
    pub y: usize,
    pub x: usize,
    pub room_type: RoomType,
    pub edges: u8,
}

// ───────── Constructors (free functions, no `impl Entity`) ─────────

// Used only as a fill-in for struct update syntax in the make_* constructors.
// Callers should never build a bare default — always go through make_entity_from_*.
pub fn get_entity_default() -> Entity {
    Entity {
        kind: EntityType::Character,
        vitals: Vitals { health: 0, health_max: 0, block: 0 },
        modifiers: modifiers_new(),
        character_name: "",
        reward_roll_offset: 0,
        monster_name: MonsterName::Cultist,
        monster_kind: MonsterKind::Normal,
        moves: &[],
        move_current: None,
        move_history: [0; MAX_MOVE_HISTORY],
        move_history_len: 0,
        dead: false,
        card_name: CardName::Strike,
        card_kind: CardKind::Attack,
        card_color: CardColor::Colorless,
        card_rarity: CardRarity::Basic,
        card_cost: 0,
        card_upgraded: false,
        card_exhaust: false,
        card_innate: false,
        card_requires_target: false,
        card_effects: &[],
        node_y: 0,
        node_x: 0,
        room_type: RoomType::CombatBoss,
        edges: 0,
    }
}

pub fn make_entity_from_character(c: Character) -> Entity {
    Entity {
        kind: EntityType::Character,
        vitals: c.vitals,
        modifiers: c.modifiers,
        character_name: c.name,
        reward_roll_offset: c.reward_roll_offset,
        ..get_entity_default()
    }
}

pub fn make_entity_from_monster(m: Monster) -> Entity {
    Entity {
        kind: EntityType::Monster,
        vitals: m.vitals,
        modifiers: m.modifiers,
        monster_name: m.name,
        monster_kind: m.monster_kind,
        moves: m.moves,
        move_current: m.move_current,
        move_history: m.move_history,
        move_history_len: m.move_history_len,
        dead: m.dead,
        ..get_entity_default()
    }
}

pub fn make_entity_from_card(c: Card) -> Entity {
    Entity {
        kind: EntityType::Card,
        card_name: c.name,
        card_kind: c.kind,
        card_color: c.color,
        card_rarity: c.rarity,
        card_cost: c.cost,
        card_upgraded: c.upgraded,
        card_exhaust: c.exhaust,
        card_innate: c.innate,
        card_requires_target: c.requires_target,
        card_effects: c.effects,
        ..get_entity_default()
    }
}

pub fn make_entity_from_room(r: Room) -> Entity {
    Entity {
        kind: EntityType::Room,
        node_y: r.y,
        node_x: r.x,
        room_type: r.room_type,
        edges: r.edges,
        ..get_entity_default()
    }
}

// ───────── Monster-history helpers (free functions) ─────────

pub fn push_move_history(entity: &mut Entity, move_idx: u8) {
    assert!(
        (entity.move_history_len as usize) < MAX_MOVE_HISTORY,
        "move_history overflow"
    );
    entity.move_history[entity.move_history_len as usize] = move_idx;
    entity.move_history_len += 1;
}

pub fn get_move_history_slice(entity: &Entity) -> &[u8] {
    &entity.move_history[..entity.move_history_len as usize]
}
