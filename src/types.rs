// Core type enums shared across the engine

use strum::EnumCount;

use crate::consts::MAX_MONSTERS;
use crate::consts::MAX_SIZE_HAND;
use crate::game::Energy;

// Vitals: physical combat state. Shared by character and monsters
#[derive(Debug, Clone, Copy)]
pub struct Vitals {
    pub health: u16,
    pub health_max: u16,
    pub block: u16,
}

// Context: mutually-exclusive game context. Tagged-union enforces that the
// player can't be in two contexts at once. Map / RestSite / Chest / GameOver
// are derived from location + room_kind + character.dead -- they have no
// transient state and thus don't need a Context variant
#[derive(Debug, Clone)]
pub enum Context {
    Combat(CombatState),
    Reward(RewardState),
    Event(EventState),
    Shop(ShopState),
}

#[derive(Debug, Clone)]
pub struct CombatState {
    pub id_hand: Vec<usize>,
    pub id_pile_draw: Vec<usize>,
    pub id_pile_discard: Vec<usize>,
    pub id_pile_exhaust: Vec<usize>,
    pub id_monsters: [usize; MAX_MONSTERS],
    pub monster_count: u8,
    pub id_monster_picked: Option<usize>,
    pub energy: Energy,
    pub this_turn_discards: u8,
    pub this_turn_attacks_played: u8,
    pub this_combat_damage_instances_taken: u8,
    pub escaped_this_combat: bool,
    pub card_last_drawn: Option<usize>,
    pub id_card_nightmare: Option<usize>,
    // Candidates for the active Discover pick. Empty when no pick is pending
    pub id_pick: Vec<usize>,
}

impl CombatState {
    pub fn new() -> Self {
        Self {
            id_hand: Vec::with_capacity(MAX_SIZE_HAND),
            id_pile_draw: Vec::with_capacity(64),
            id_pile_discard: Vec::with_capacity(64),
            id_pile_exhaust: Vec::with_capacity(32),
            id_monsters: [0; MAX_MONSTERS],
            monster_count: 0,
            id_monster_picked: None,
            energy: Energy { current: 3, max: 3 },
            this_turn_discards: 0,
            this_turn_attacks_played: 0,
            this_combat_damage_instances_taken: 0,
            escaped_this_combat: false,
            card_last_drawn: None,
            id_card_nightmare: None,
            id_pick: Vec::new(),
        }
    }
}


#[derive(Debug, Clone)]
pub struct RewardState {
    pub id_cards: Vec<usize>,
    pub id_relic: Option<usize>,
    pub id_potion: Option<usize>,
    pub gold: Option<u16>,
}

#[derive(Debug, Clone, Copy)]
pub struct EventState {
    pub id_event: usize,
}

#[derive(Debug, Clone, Default)]
pub struct ShopState {}

pub const ZERO_VITALS: Vitals = Vitals {
    health: 0,
    health_max: 0,
    block: 0,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount)]
#[repr(u8)]
pub enum CardName {
    AThousandCuts,
    Accuracy,
    Acrobatics,
    Adrenaline,
    AfterImage,
    Alchemize,
    AllOutAttack,
    Backflip,
    Backstab,
    BandageUp,
    Bane,
    BladeDance,
    Blind,
    Blur,
    BouncingFlask,
    BulletTime,
    Burn,
    Burst,
    CalculatedGamble,
    Caltrops,
    Catalyst,
    Choke,
    CloakAndDagger,
    Concentrate,
    CorpseExplosion,
    CripplingPoison,
    DaggerSpray,
    DaggerThrow,
    Dash,
    Dazed,
    DeadlyPoison,
    DeepBreath,
    Defend,
    Deflect,
    DieDieDie,
    Distraction,
    DodgeAndRoll,
    Doppelganger,
    EndlessAgony,
    Envenom,
    EscapePlan,
    Eviscerate,
    Expertise,
    Finesse,
    Finisher,
    FlashOfSteel,
    Flechettes,
    FlyingKnee,
    Footwork,
    GlassKnife,
    GoodInstincts,
    GrandFinale,
    HeelHook,
    InfiniteBlades,
    LegSweep,
    Malaise,
    MasterOfStrategy,
    MasterfulStab,
    MindBlast,
    Neutralize,
    Nightmare,
    NoxiousFumes,
    Outmaneuver,
    PhantasmalKiller,
    PiercingWail,
    PoisonedStab,
    Predator,
    Prepared,
    QuickSlash,
    Reflex,
    RiddleWithHoles,
    Setup,
    Shiv,
    Skewer,
    Slice,
    Slimed,
    SneakyStrike,
    StormOfSteel,
    Strike,
    SuckerPunch,
    Survivor,
    SwiftStrike,
    Tactician,
    Terror,
    ToolsOfTheTrade,
    Unload,
    WellLaidPlans,
    WraithForm,

    // Curses
    AscendersBane,
    Regret,
    Pain,
    Doubt,
    Decay,
    Injury,
    Shame,
    Writhe,
    Parasite,
    Normality,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardKind {
    Attack,
    Curse,
    Power,
    Skill,
    Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardColor {
    Colorless,
    Curse,
    Green,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardRarity {
    Basic,
    Common,
    Uncommon,
    Rare,
    Special,
    Curse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount)]
#[repr(u8)]
pub enum MonsterName {
    Cultist,
    FungiBeast,
    GremlinFat,
    GremlinNob,
    GremlinThief,
    GremlinTsundere,
    GremlinWarrior,
    GremlinWizard,
    Hexaghost,
    JawWorm,
    Lagavulin,
    Looter,
    LouseDefensive,
    LouseNormal,
    Sentry,
    SlaverBlue,
    SlaverRed,
    SlimeAcidLarge,
    SlimeAcidMedium,
    SlimeAcidSmall,
    SlimeBoss,
    SlimeSpikeLarge,
    SlimeSpikeMedium,
    SlimeSpikeSmall,
    TheGuardian,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MonsterKind {
    Normal,
    Elite,
    Boss,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncounterPool {
    Act1Easy,
    Act1Hard,
    Act1Elite,
    Act1Boss,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount)]
#[repr(u8)]
pub enum MonsterEncounter {
    // Easy
    Cultist,
    JawWorm,
    TwoLouse,
    SmallSlimes,

    // Hard
    BlueSlaver,
    RedSlaver,
    Looter,
    TwoFungiBeasts,
    ThreeLouse,
    LargeSlime,
    LotsOfSlimes,
    GremlinGang,
    ExordiumThugs,
    ExordiumWildlife,

    // Elite
    GremlinNob,
    Lagavulin,
    ThreeSentries,

    // Boss
    TheGuardian,
    Hexaghost,
    SlimeBoss,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoomKind {
    CombatBoss,
    CombatElite,
    CombatMonster,
    RestSite,
    Treasure,
    EventRoom,
    Shop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount)]
#[repr(u8)]
pub enum EventName {
    BigFish,
    Cleric,
    Designer,
    Duplicator,
    GoldShrine,
    GoldenIdolEvent,
    GoldenWing,
    GoopPuddle,
    LivingWall,
    PurificationShrine,
    ScrapOoze,
    ShiningLight,
    Sssserpent,
    Transmogrifier,
    UpgradeShrine,
    WeMeetAgain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeckSelectKind {
    Remove,
    UpgradeAny,
    TransformOne,
    DuplicateAny,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HandSelectKind {
    Discard,
    Retain,
    Setup,
    Nightmare,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChestKind {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount)]
#[repr(u8)]
pub enum PotionName {
    EnergyPotion,
    BlockPotion,
    StrengthPotion,
    DexterityPotion,
    FirePotion,
    ExplosivePotion,
    WeakPotion,
    FearPotion,
    PoisonPotion,
    SwiftPotion,
    AttackPotion,
    SkillPotion,
    PowerPotion,
    FruitJuice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PotionRarity {
    Common,
    Uncommon,
    Rare,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount)]
#[repr(u8)]
pub enum RelicName {
    SnakeRing = 0,
    Akabeko,
    Anchor,
    BagOfMarbles,
    BagOfPreparation,
    BloodVial,
    BronzeScales,
    Kunai,
    NinjaScroll,
    OddlySmoothStone,
    Shuriken,
    ThreadAndNeedle,
    TwistedFunnel,
    Vajra,
    Circlet,
}

impl RelicName {
    pub fn from_u8(v: u8) -> Self {
        assert!((v as usize) < RelicName::COUNT, "invalid RelicName: {v}");
        // SAFETY: repr(u8) and we validated the range
        unsafe { std::mem::transmute(v) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelicTier {
    Starter,
    Common,
    Uncommon,
    Rare,
    Boss,
    Shop,
    Special,
}

