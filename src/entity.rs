// Entities: every kind of thing that lives in `GameState.entities`
//
// One flat "fat" Entity struct holds all fields from all kinds. A runtime
// `EntityKind` tag distinguishes them. Variant-specific `const fn`
// constructors below (`make_entity_card`, `make_entity_monster`, etc.) are the only
// way to build an Entity — they set the relevant fields and zero the rest

use crate::consts::{MAX_MOVE_HISTORY, MAX_SIZE_HAND};
use crate::effect::{Effect, ZERO_EFFECT};
use crate::modifier::{Modifiers, ZERO_MODIFIERS};

// Per-card effect array capacity. Largest current card is RiddleWithHoles
// (5 hits). 8 leaves headroom for Tier 5 cards (Eviscerate × 3, Skewer × X
// with practical caps, etc.). Bump when a card legitimately exceeds it
pub const MAX_EFFECTS_PER_CARD: usize = 8;
use crate::types::{
    CardColor, CardKind, CardName, CardRarity, ChestKind, MonsterKind, MonsterName, RelicName,
    RelicTier, RoomKind, Vitals, ZERO_VITALS,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityKind {
    Character,
    Monster,
    Card,
    Room,
    Relic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayRestriction {
    Always,        // Standard cards. Playable iff the energy cost is met
    Never,         // Permanently unplayable (curses, statuses, Reflex, Tactician, etc.)
    DrawPileEmpty, // Playable iff the draw pile is empty (Grand Finale only)
}

// XCost.offset is consumed by the per-play multiplier in `process_effect_card_play`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardCostKind {
    Fixed,
    MinusDiscardsThisTurn,
    GrowsOnDamageInstanceTaken,
    XCost { offset: i8 },
}

#[derive(Debug, Clone, Copy)]
pub enum Intent {
    Attack { damage: u16, instances: u8 },
    AttackBlock { damage: u16, instances: u8 },
    AttackBuff { damage: u16, instances: u8 },
    AttackDebuff { damage: u16, instances: u8 },
    Block,
    BlockBuff,
    Buff,
    Debuff,
    DebuffPowerful,
    Escape,
    Sleep,
    Stunned,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub name: &'static str,
    pub effects: &'static [Effect],
    pub intent: Intent,
}

// Fat Entity
#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub kind: EntityKind,

    // Combatant (Character or Monster)
    pub vitals: Vitals,
    pub modifiers: Modifiers,
    pub dead: bool,

    // Character-only
    pub character_name: &'static str,
    pub character_reward_roll_offset: i8,
    pub character_gold: u16,

    // Monster-only
    pub monster_name: MonsterName,
    pub monster_kind: MonsterKind,
    pub moves: &'static [Move],
    pub move_current: Option<usize>,
    pub move_history: [u8; MAX_MOVE_HISTORY],
    pub move_history_len: u8,
    pub monster_cycle_count: u8,  // Only used by "The Guardian"
    pub monster_stolen_gold: u16, // Only used by "Looter"

    // Card-only
    pub card_name: CardName,
    pub card_kind: CardKind,
    pub card_color: CardColor,
    pub card_rarity: CardRarity,
    pub card_cost: u8,
    pub card_upgraded: bool,
    pub card_exhaust: bool,
    pub card_ethereal: bool,
    pub card_innate: bool,
    pub card_requires_target: bool,
    pub card_retain: bool,
    pub card_play_restriction: PlayRestriction,
    pub card_free_to_play_once: bool,
    pub card_cost_kind: CardCostKind,
    pub card_cost_override: Option<u8>,
    pub card_effects: [Effect; MAX_EFFECTS_PER_CARD],
    pub card_effects_len: u8,
    pub card_on_discard_effects: &'static [Effect],
    pub card_on_draw_effects: &'static [Effect],

    // Room-only
    pub room_y: usize,
    pub room_x: usize,
    pub room_kind: RoomKind,
    pub edges: u8,
    pub room_chest_kind: Option<ChestKind>,

    // Relic-only
    pub relic_name: RelicName,
    pub relic_tier: RelicTier,
    pub relic_counter: i16,
    pub relic_used_up: bool,
    pub relic_effects_on_combat_start: &'static [Effect],
}

// Private zero-fill used by the public const fn constructors below
// Not exported — external code must go through one of the `*_entity` fns
const ZERO_ENTITY: Entity = Entity {
    kind: EntityKind::Character,
    vitals: ZERO_VITALS,
    modifiers: ZERO_MODIFIERS,
    character_name: "",
    character_reward_roll_offset: 0,
    character_gold: 0,
    monster_stolen_gold: 0,
    monster_name: MonsterName::Cultist,
    monster_kind: MonsterKind::Normal,
    moves: &[],
    move_current: None,
    move_history: [0; MAX_MOVE_HISTORY],
    move_history_len: 0,
    monster_cycle_count: 0,
    dead: false,
    card_name: CardName::Strike,
    card_kind: CardKind::Attack,
    card_color: CardColor::Colorless,
    card_rarity: CardRarity::Basic,
    card_cost: 0,
    card_upgraded: false,
    card_exhaust: false,
    card_ethereal: false,
    card_innate: false,
    card_requires_target: false,
    card_retain: false,
    card_play_restriction: PlayRestriction::Always,
    card_free_to_play_once: false,
    card_cost_kind: CardCostKind::Fixed,
    card_cost_override: None,
    card_effects: [ZERO_EFFECT; MAX_EFFECTS_PER_CARD],
    card_effects_len: 0,
    card_on_discard_effects: &[],
    card_on_draw_effects: &[],
    room_y: 0,
    room_x: 0,
    room_kind: RoomKind::CombatBoss,
    edges: 0,
    room_chest_kind: None,
    relic_name: RelicName::SnakeRing,
    relic_tier: RelicTier::Starter,
    relic_counter: 0,
    relic_used_up: false,
    relic_effects_on_combat_start: &[],
};

// Constructors
pub const fn make_entity_character(
    name: &'static str,
    vitals: Vitals,
    character_reward_roll_offset: i8,
    character_gold: u16,
) -> Entity {
    Entity {
        kind: EntityKind::Character,
        vitals,
        character_name: name,
        character_reward_roll_offset,
        character_gold,
        ..ZERO_ENTITY
    }
}

pub const fn make_entity_monster(
    name: MonsterName,
    monster_kind: MonsterKind,
    vitals: Vitals,
    modifiers: Modifiers,
    moves: &'static [Move],
) -> Entity {
    Entity {
        kind: EntityKind::Monster,
        vitals,
        modifiers,
        monster_name: name,
        monster_kind,
        moves,
        ..ZERO_ENTITY
    }
}

#[allow(clippy::too_many_arguments)]
pub const fn make_entity_card(
    name: CardName,
    kind: CardKind,
    color: CardColor,
    rarity: CardRarity,
    cost: u8,
    cost_kind: CardCostKind,
    upgraded: bool,
    exhaust: bool,
    ethereal: bool,
    innate: bool,
    requires_target: bool,
    effects: &[Effect],
    on_discard_effects: &'static [Effect],
    on_draw_effects: &'static [Effect],
    play_restriction: PlayRestriction,
) -> Entity {
    assert!(
        effects.len() <= MAX_EFFECTS_PER_CARD,
        "card_effects exceeds MAX_EFFECTS_PER_CARD",
    );
    let mut arr = [ZERO_EFFECT; MAX_EFFECTS_PER_CARD];
    let mut i = 0;
    while i < effects.len() {
        arr[i] = effects[i];
        i += 1;
    }
    Entity {
        kind: EntityKind::Card,
        card_name: name,
        card_kind: kind,
        card_color: color,
        card_rarity: rarity,
        card_cost: cost,
        card_cost_kind: cost_kind,
        card_upgraded: upgraded,
        card_exhaust: exhaust,
        card_ethereal: ethereal,
        card_innate: innate,
        card_requires_target: requires_target,
        card_play_restriction: play_restriction,
        card_effects: arr,
        card_effects_len: effects.len() as u8,
        card_on_discard_effects: on_discard_effects,
        card_on_draw_effects: on_draw_effects,
        ..ZERO_ENTITY
    }
}

pub const fn make_entity_room(y: usize, x: usize, room_kind: RoomKind, edges: u8) -> Entity {
    Entity {
        kind: EntityKind::Room,
        room_y: y,
        room_x: x,
        room_kind,
        edges,
        ..ZERO_ENTITY
    }
}

pub const fn make_entity_relic(
    name: RelicName,
    tier: RelicTier,
    counter_init: i16,
    effects_on_combat_start: &'static [Effect],
) -> Entity {
    Entity {
        kind: EntityKind::Relic,
        relic_name: name,
        relic_tier: tier,
        relic_counter: counter_init,
        relic_used_up: false,
        relic_effects_on_combat_start: effects_on_combat_start,
        ..ZERO_ENTITY
    }
}

pub fn card_effective_cost(
    card: &Entity,
    this_turn_discards: u8,
    this_combat_damage_instances_taken: u8,
    energy_current: u8,
) -> u8 {
    if card.card_free_to_play_once {
        return 0;
    }
    if let Some(override_) = card.card_cost_override {
        return override_;
    }
    match card.card_cost_kind {
        CardCostKind::Fixed => card.card_cost,
        CardCostKind::MinusDiscardsThisTurn => card.card_cost.saturating_sub(this_turn_discards),
        CardCostKind::GrowsOnDamageInstanceTaken => card
            .card_cost
            .saturating_add(this_combat_damage_instances_taken),
        CardCostKind::XCost { .. } => energy_current,
    }
}

// Used by Distraction, EndlessAgony copy spawning, and the CardDraw cap branch
pub fn add_card_to_hand_or_discard(
    entities: &mut Vec<Entity>,
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
    card: Entity,
) -> usize {
    let id_card = entities.len();
    entities.push(card);
    if id_hand.len() < MAX_SIZE_HAND {
        id_hand.push(id_card);
    } else {
        id_pile_discard.push(id_card);
    }
    id_card
}

// Evaluate a PlayRestriction against the relevant slice of game state
pub fn is_play_restriction_satisfied(restriction: PlayRestriction, id_pile_draw: &[usize]) -> bool {
    match restriction {
        PlayRestriction::Always => true,
        PlayRestriction::Never => false,
        PlayRestriction::DrawPileEmpty => id_pile_draw.is_empty(),
    }
}

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
