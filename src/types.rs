use strum::EnumCount;

// Vitals: physical combat state. Shared by character and monsters
#[derive(Debug, Clone, Copy)]
pub struct Vitals {
    pub health: u16,
    pub health_max: u16,
    pub block: u16,
}

// Direction of a quantity change (health, gold, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeltaSign {
    Gain,
    Loss,
}

// Persistent screen; transient working memory lives in flat GameState fields
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Screen {
    Combat,
    Reward,
    Event,
    Shop,
    Map,
    RestSite,
    Chest,
}

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
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount)]
#[repr(u8)]
pub enum EventName {
    BigFish,
    TheCleric,
    Duplicator,
    GoldenShrine,
    GoldenIdol,
    WingStatue,
    WorldOfGoop,
    LivingWall,
    Purifier,
    ScrapOoze,
    ShiningLight,
    TheSsssserpent,
    Transmogrifier,
    UpgradeShrine,
    TheDivineFountain,
    TheLab,
    TheWomanInBlue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChestKind {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RewardKind {
    Card,
    Relic,
    Potion,
    Gold,
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
    GoldenIdol,
    Lantern,
    ClockworkSouvenir,
    GremlinVisage,
    RedMask,
    Nunchaku,
    InkBottle,
    LetterOpener,
    OrnamentalFan,
    BirdFacedUrn,
    MummifiedHand,
    OrangePellets,
    StrangeSpoon,
    ChemicalX,
    ArtOfWar,
    Orichalcum,
    Pocketwatch,
    StoneCalendar,
    Abacus,
    Sundial,
    WhiteBeastStatue,
    DollysMirror,
    LeesWaffle,
    HappyFlower,
    IncenseBurner,
    MercuryHourglass,
    HornCleat,
    CaptainsWheel,
    Calipers,
    IceCream,
    SneckoSkull,
    Ginger,
    Turnip,
    Tingsha,
    ToughBandages,
    GremlinHorn,
    TheSpecimen,
    LizardTail,
    Boot,
    Torii,
    TungstenRod,
    HandDrill,
    StrikeDummy,
    PaperKrane,
    CentennialPuzzle,
    MealTicket,
    MawBank,
    JuzuBracelet,
    TinyChest,
    EternalFeather,
    AncientTeaSet,
    RegalPillow,
    MeatOnTheBone,
    Omamori,
    DarkstonePeriapt,
    CeramicFish,
    FrozenEgg,
    MoltenEgg,
    ToxicEgg,
    ToyOrnithopter,
    SmilingMask,
    DeadBranch,
    DuVuDoll,
    Pantograph,
    SlingOfCourage,
    Strawberry,
    Pear,
    Mango,
    OldCoin,
    PotionBelt,
    WarPaint,
    Whetstone,
    EmptyCage,
    PandorasBox,
    PenNib,
    FossilizedHelix,
    PreservedInsect,
    UnceasingTop,
    BlueCandle,
    MedicalKit,
}

pub fn relic_name_from_u8(v: u8) -> RelicName {
    assert!((v as usize) < RelicName::COUNT, "invalid RelicName: {v}");
    // SAFETY: repr(u8) and we validated the range
    unsafe { std::mem::transmute(v) }
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
