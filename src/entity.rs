// Fat Entity + EntityKind tag; build only via `make_entity_*` const fns

use strum::EnumCount;

use crate::consts::MAX_EFFECTS_PER_CARD;
use crate::consts::MAX_EFFECTS_PER_MOVE;
use crate::consts::MAX_MOVE_HISTORY;
use crate::consts::MAX_MOVES_PER_MONSTER;
use crate::effect::Effect;
use crate::effect::ZERO_EFFECT;
use crate::modifier::Modifiers;
use crate::modifier::ZERO_MODIFIERS;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;
use crate::types::ChestKind;
use crate::types::CostScope;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::PotionName;
use crate::types::PotionRarity;
use crate::types::RelicName;
use crate::types::RelicTier;
use crate::types::RoomKind;
use crate::types::Vitals;
use crate::types::ZERO_VITALS;
use crate::utils::has_relic;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityKind {
    Character,
    Monster,
    Card,
    Room,
    Relic,
    Potion,
    EventOption,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayRestriction {
    Always,        // Standard cards. Playable iff the energy cost is met
    Never,         // Permanently unplayable (curses, statuses, Reflex, Tactician, etc.)
    DrawPileEmpty, // Playable iff the draw pile is empty (Grand Finale only)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CostOverride {
    pub amount: u8,
    pub scope: CostScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardCostKind {
    Fixed,
    MinusDiscardsThisTurn,
    GrowsOnDamageInstanceTaken,
    XCost { offset: i8 }, // offset is consumed by the per-play multiplier in `process_effect_card_play`
}

// TODO: revisit implementation. Could be flat-enum and `damage: u16` and `instances: u8`
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
    pub effects: [Effect; MAX_EFFECTS_PER_MOVE],
    pub effects_len: u8,
    pub intent: Intent,
}

// Zero-fill sentinel; pads slots past effects_len / monster_moves_len
pub const ZERO_MOVE: Move = Move {
    name: "",
    effects: [ZERO_EFFECT; MAX_EFFECTS_PER_MOVE],
    effects_len: 0,
    intent: Intent::Unknown,
};

pub const fn make_move(name: &'static str, effects: &[Effect], intent: Intent) -> Move {
    assert!(
        effects.len() <= MAX_EFFECTS_PER_MOVE,
        "move effects exceeds MAX_EFFECTS_PER_MOVE",
    );
    let mut arr = [ZERO_EFFECT; MAX_EFFECTS_PER_MOVE];
    let mut i = 0;
    while i < effects.len() {
        arr[i] = effects[i];
        i += 1;
    }
    Move {
        name,
        effects: arr,
        effects_len: effects.len() as u8,
        intent,
    }
}

// Fat Entity
#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub kind: EntityKind,

    // Combatant — Character, Monster
    pub vitals: Vitals,
    pub modifiers: Modifiers,
    pub dead: bool,

    // Card and Potion (player-played entities that may pick a monster target)
    pub requires_target: bool,

    // Character-only
    pub character_name: &'static str,
    pub character_reward_roll_offset: i8,
    pub character_gold: u16,

    // Monster-only
    pub monster_name: MonsterName,
    pub monster_kind: MonsterKind,
    pub monster_moves: [Move; MAX_MOVES_PER_MONSTER],
    pub monster_moves_len: u8,
    pub monster_move_current: Option<usize>,
    pub monster_move_history: [u8; MAX_MOVE_HISTORY],
    pub monster_move_history_len: u8,
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
    pub card_bottled: bool,
    pub card_retain: bool,
    pub card_play_restriction: PlayRestriction,

    pub card_cost_kind: CardCostKind,
    // Paired amount+lifetime; Turn clears at turn end, UntilPlayed clears on play
    pub card_cost_override: Option<CostOverride>,
    pub card_effects: [Effect; MAX_EFFECTS_PER_CARD],
    pub card_effects_len: u8,
    pub card_on_discard_effects: &'static [Effect],
    pub card_on_draw_effects: &'static [Effect],

    // Room-only
    pub room_y: usize,
    pub room_x: usize,
    pub room_kind: RoomKind,
    pub room_edges: u8,
    pub room_chest_kind: Option<ChestKind>,
    pub room_chest_opened: bool,
    pub room_rest_site_done: bool,
    pub room_shop_purged: bool,

    // Relic-only
    pub relic_name: RelicName,
    pub relic_tier: RelicTier,
    pub relic_counter: i16,

    // Shop price while stocked; stale after purchase (nothing reads it outside Shop mode)
    pub price: u16,
    pub relic_used_up: bool,
    pub relic_effects_on_combat_start: &'static [Effect],

    // Potion-only
    pub potion_name: PotionName,
    pub potion_rarity: PotionRarity,
    pub potion_combat_only: bool,
    pub potion_effects: &'static [Effect],

    // EventOption-only
    pub event_option_label: &'static str,
    pub event_option_effects: &'static [Effect],
}

// Zero-fill sentinel; used by const constructors and unused arena slots
pub const ZERO_ENTITY: Entity = Entity {
    kind: EntityKind::Character,
    vitals: ZERO_VITALS,
    modifiers: ZERO_MODIFIERS,
    character_name: "",
    character_reward_roll_offset: 0,
    character_gold: 0,
    monster_stolen_gold: 0,
    monster_name: MonsterName::Cultist,
    monster_kind: MonsterKind::Normal,
    monster_moves: [ZERO_MOVE; MAX_MOVES_PER_MONSTER],
    monster_moves_len: 0,
    monster_move_current: None,
    monster_move_history: [0; MAX_MOVE_HISTORY],
    monster_move_history_len: 0,
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
    card_bottled: false,
    requires_target: false,
    card_retain: false,
    card_play_restriction: PlayRestriction::Always,
    card_cost_kind: CardCostKind::Fixed,
    card_cost_override: None,
    card_effects: [ZERO_EFFECT; MAX_EFFECTS_PER_CARD],
    card_effects_len: 0,
    card_on_discard_effects: &[],
    card_on_draw_effects: &[],
    room_y: 0,
    room_x: 0,
    room_kind: RoomKind::CombatBoss,
    room_edges: 0,
    room_chest_kind: None,
    room_chest_opened: false,
    room_rest_site_done: false,
    room_shop_purged: false,
    relic_name: RelicName::SnakeRing,
    relic_tier: RelicTier::Starter,
    relic_counter: 0,
    price: 0,
    relic_used_up: false,
    relic_effects_on_combat_start: &[],
    potion_name: PotionName::EnergyPotion,
    potion_rarity: PotionRarity::Common,
    potion_combat_only: true,
    potion_effects: &[],
    event_option_label: "",
    event_option_effects: &[],
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
    moves: &[Move],
) -> Entity {
    assert!(
        moves.len() <= MAX_MOVES_PER_MONSTER,
        "monster_moves exceeds MAX_MOVES_PER_MONSTER",
    );
    let mut arr = [ZERO_MOVE; MAX_MOVES_PER_MONSTER];
    let mut i = 0;
    while i < moves.len() {
        arr[i] = moves[i];
        i += 1;
    }
    Entity {
        kind: EntityKind::Monster,
        vitals,
        modifiers,
        monster_name: name,
        monster_kind,
        monster_moves: arr,
        monster_moves_len: moves.len() as u8,
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
        requires_target,
        card_play_restriction: play_restriction,
        card_effects: arr,
        card_effects_len: effects.len() as u8,
        card_on_discard_effects: on_discard_effects,
        card_on_draw_effects: on_draw_effects,
        ..ZERO_ENTITY
    }
}

pub const fn make_entity_room(y: usize, x: usize, room_kind: RoomKind, room_edges: u8) -> Entity {
    Entity {
        kind: EntityKind::Room,
        room_y: y,
        room_x: x,
        room_kind,
        room_edges,
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

pub const fn make_entity_potion(
    name: PotionName,
    rarity: PotionRarity,
    requires_target: bool,
    combat_only: bool,
    effects: &'static [Effect],
) -> Entity {
    Entity {
        kind: EntityKind::Potion,
        potion_name: name,
        potion_rarity: rarity,
        requires_target: requires_target,
        potion_combat_only: combat_only,
        potion_effects: effects,
        ..ZERO_ENTITY
    }
}

pub const fn make_entity_event_option(label: &'static str, effects: &'static [Effect]) -> Entity {
    Entity {
        kind: EntityKind::EventOption,
        event_option_label: label,
        event_option_effects: effects,
        ..ZERO_ENTITY
    }
}

pub fn get_card_effective_cost(
    card: &Entity,
    this_turn_discards: u8,
    this_combat_damage_instances_taken: u8,
    energy_current: u8,
) -> u8 {
    if let Some(override_) = card.card_cost_override {
        return override_.amount;
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

// Evaluate a PlayRestriction against the relevant slice of game state
pub fn is_play_restriction_satisfied(
    restriction: PlayRestriction,
    card_kind: CardKind,
    id_pile_draw: &[usize],
    id_relics: &[Option<usize>; RelicName::COUNT],
) -> bool {
    match restriction {
        PlayRestriction::Always => true,
        PlayRestriction::Never => match card_kind {
            CardKind::Curse => has_relic(id_relics, RelicName::BlueCandle),
            CardKind::Status => has_relic(id_relics, RelicName::MedicalKit),
            _ => false,
        },
        PlayRestriction::DrawPileEmpty => id_pile_draw.is_empty(),
    }
}

pub fn push_move_history(entity: &mut Entity, move_idx: u8) {
    let len = entity.monster_move_history_len as usize;
    if len < MAX_MOVE_HISTORY {
        entity.monster_move_history[len] = move_idx;
        entity.monster_move_history_len += 1;
    } else {
        // Marathon combat: drop the oldest, keep the last MAX_MOVE_HISTORY moves
        entity.monster_move_history.copy_within(1.., 0);
        entity.monster_move_history[MAX_MOVE_HISTORY - 1] = move_idx;
    }
}

pub fn get_move_history_slice(entity: &Entity) -> &[u8] {
    &entity.monster_move_history[..entity.monster_move_history_len as usize]
}
