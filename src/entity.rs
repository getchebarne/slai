// Entities: every kind of thing that lives in `GameState.entities`.
//
// One flat "fat" Entity struct holds all fields from all kinds. A runtime
// `EntityKind` tag distinguishes them. Variant-specific `const fn`
// constructors below (`make_entity_card`, `make_entity_monster`, etc.) are the only
// way to build an Entity — they set the relevant fields and zero the rest.

use crate::consts::MAX_MOVE_HISTORY;
use crate::effect::{Effect, ZERO_EFFECT};
use crate::modifier::{Modifiers, ZERO_MODIFIERS};

// Per-card effect array capacity. Largest current card is RiddleWithHoles
// (5 hits). 8 leaves headroom for Tier 5 cards (Eviscerate × 3, Skewer × X
// with practical caps, etc.). Bump when a card legitimately exceeds it.
pub const MAX_EFFECTS_PER_CARD: usize = 8;
use crate::types::{
    CardColor, CardKind, CardName, CardRarity, MonsterKind, MonsterName, RoomKind, Vitals,
    ZERO_VITALS,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityKind {
    Character,
    Monster,
    Card,
    Room,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayRestriction {
    Always,        // Standard cards. Playable iff the energy cost is met
    Never,         // Permanently unplayable (curses, statuses, Reflex, Tactician, etc.)
    DrawPileEmpty, // Playable iff the draw pile is empty (Grand Finale only)
}

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
    pub reward_roll_offset: i8,

    // Monster-only
    pub monster_name: MonsterName,
    pub monster_kind: MonsterKind,
    pub moves: &'static [Move],
    pub move_current: Option<usize>,
    pub move_history: [u8; MAX_MOVE_HISTORY],
    pub move_history_len: u8,
    pub cycle_count: u8, // Only used by "The Guardian"

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
    pub card_retain: bool,
    pub card_play_restriction: PlayRestriction,
    /// One-shot free-play flag set by Setup (and Distraction). When true,
    /// the next play of this card instance ignores energy cost and clears
    /// the flag. Per-instance — does not propagate to other copies of the
    /// same card name (unless those copies are themselves snapshotted, e.g.
    /// via Nightmare on a Setup-flagged card).
    pub card_free_to_play_once: bool,
    /// Per-instance card effects, mutable. Stored inline so each Entity
    /// owns its own copy — needed for cards like GlassKnife that mutate
    /// their own damage values across plays. Slots past `card_effects_len`
    /// are ignored. Read via `&card.card_effects[..card.card_effects_len as usize]`.
    pub card_effects: [Effect; MAX_EFFECTS_PER_CARD],
    pub card_effects_len: u8,
    /// Effects to push when this card is discarded by an explicit
    /// `EffectKind::CardDiscard` (player or card-induced). NOT triggered by
    /// `CardMoveToDiscard` (after-play move) or `CardDiscardEndOfTurn`.
    /// Used by Reflex (draw 2/3) and Tactician (gain 1/2 energy). Static —
    /// not mutated per-instance, so no array-by-value overhead.
    pub card_on_discard_effects: &'static [Effect],

    // Room-only
    pub room_y: usize,
    pub room_x: usize,
    pub room_kind: RoomKind,
    pub edges: u8,
}

// Private zero-fill used by the public const fn constructors below.
// Not exported — external code must go through one of the `*_entity` fns.
const ZERO_ENTITY: Entity = Entity {
    kind: EntityKind::Character,
    vitals: ZERO_VITALS,
    modifiers: ZERO_MODIFIERS,
    character_name: "",
    reward_roll_offset: 0,
    monster_name: MonsterName::Cultist,
    monster_kind: MonsterKind::Normal,
    moves: &[],
    move_current: None,
    move_history: [0; MAX_MOVE_HISTORY],
    move_history_len: 0,
    cycle_count: 0,
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
    card_retain: false,
    card_play_restriction: PlayRestriction::Always,
    card_free_to_play_once: false,
    card_effects: [ZERO_EFFECT; MAX_EFFECTS_PER_CARD],
    card_effects_len: 0,
    card_on_discard_effects: &[],
    room_y: 0,
    room_x: 0,
    room_kind: RoomKind::CombatBoss,
    edges: 0,
};

// Constructors
pub const fn make_entity_character(
    name: &'static str,
    vitals: Vitals,
    reward_roll_offset: i8,
) -> Entity {
    Entity {
        kind: EntityKind::Character,
        vitals,
        character_name: name,
        reward_roll_offset,
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
    upgraded: bool,
    exhaust: bool,
    innate: bool,
    requires_target: bool,
    effects: &[Effect],
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
        card_upgraded: upgraded,
        card_exhaust: exhaust,
        card_innate: innate,
        card_requires_target: requires_target,
        card_play_restriction: play_restriction,
        card_effects: arr,
        card_effects_len: effects.len() as u8,
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

// Effective energy cost for a card right now: 0 if its `card_free_to_play_once`
// flag is set (Setup/Distraction), otherwise the card's normal cost. The flag
// is consumed by `process_effect_card_play` when the card actually resolves.
pub fn card_effective_cost(card: &Entity) -> u8 {
    if card.card_free_to_play_once { 0 } else { card.card_cost }
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
