use crate::consts::MAX_EFFECTS_PER_CARD;
use crate::consts::MAX_EFFECTS_PER_EVENT_OPTION;
use crate::consts::MAX_EFFECTS_PER_MOVE;
use crate::consts::MAX_MOVE_HISTORY;
use crate::effect::EFFECT_ZERO;
use crate::effect::Effect;
use crate::modifier::MODIFIERS_ZERO;
use crate::modifier::Modifiers;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;
use crate::types::CostScope;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::PotionName;
use crate::types::PotionRarity;
use crate::types::RelicName;
use crate::types::RelicTier;
use crate::types::RoomKind;
use crate::types::VITALS_ZERO;
use crate::types::Vitals;

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
    Always,        // Standard Cards. Playable iff the energy cost is met
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

// Fat Entity
#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub kind: EntityKind,

    // Combatant: Character or Monster
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
    pub monster_moves: &'static [Move],
    pub monster_move_damage_override: Option<u16>, // Only used by "Hexaghost"
    pub monster_move_current: Option<usize>,
    pub monster_move_history: [u8; MAX_MOVE_HISTORY],
    pub monster_move_history_len: u8,
    pub monster_cycle_count: u8,  // Only used by "The Guardian"
    pub monster_stolen_gold: u16, // Only used by "Looter" and "Mugger"

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
    pub card_cost_override: Option<CostOverride>,
    pub card_effects: [Effect; MAX_EFFECTS_PER_CARD],
    pub card_effects_len: u8,
    pub card_on_discard_effects: &'static [Effect],
    pub card_effects_on_draw: &'static [Effect],

    // Room-only
    pub room_y: usize,
    pub room_x: usize,
    pub room_kind: RoomKind,
    pub room_edges: u8,

    // Relic-only
    pub relic_name: RelicName,
    pub relic_tier: RelicTier,
    pub relic_counter: i16,
    pub relic_used_up: bool,
    pub relic_seq: u16,
    pub relic_effects_combat_start: &'static [Effect],

    // Potion-only
    pub potion_name: PotionName,
    pub potion_rarity: PotionRarity,
    pub potion_combat_only: bool,
    pub potion_effects: &'static [Effect],

    // EventOption-only
    pub event_option_effects: [Effect; MAX_EFFECTS_PER_EVENT_OPTION],
    pub event_option_effects_len: u8,
}

// Zero-fill sentinel; used by const constructors and unused arena slots
pub const ENTITY_ZERO: Entity = Entity {
    kind: EntityKind::Character,
    vitals: VITALS_ZERO,
    modifiers: MODIFIERS_ZERO,
    character_name: "",
    character_reward_roll_offset: 0,
    character_gold: 0,
    monster_stolen_gold: 0,
    monster_name: MonsterName::Cultist,
    monster_kind: MonsterKind::Normal,
    monster_moves: &[],
    monster_move_damage_override: None,
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
    card_retain: false,
    card_play_restriction: PlayRestriction::Always,
    card_cost_kind: CardCostKind::Fixed,
    card_cost_override: None,
    card_effects: [EFFECT_ZERO; MAX_EFFECTS_PER_CARD],
    card_effects_len: 0,
    card_on_discard_effects: &[],
    card_effects_on_draw: &[],
    room_y: 0,
    room_x: 0,
    room_kind: RoomKind::CombatBoss,
    room_edges: 0,
    relic_name: RelicName::RingOfTheSnake,
    relic_tier: RelicTier::Starter,
    relic_counter: 0,
    relic_used_up: false,
    relic_seq: 0,
    relic_effects_combat_start: &[],
    potion_name: PotionName::Energy,
    potion_rarity: PotionRarity::Common,
    potion_combat_only: true,
    potion_effects: &[],
    event_option_effects: [EFFECT_ZERO; MAX_EFFECTS_PER_EVENT_OPTION],
    event_option_effects_len: 0,
};
