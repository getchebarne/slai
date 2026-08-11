use strum::EnumCount;

use crate::consts::MAX_MONSTERS;
use crate::effect::EventLoot;
use crate::events::EventKind;

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

// The game's mode: each variant owns the working memory that is only meaningful
// while it is active; constructed whole at entry, destroyed by variant replacement
#[derive(Debug, Clone)]
pub enum Mode {
    Combat {
        id_hand: Vec<usize>,
        id_pile_draw: Vec<usize>,
        id_pile_discard: Vec<usize>,
        id_pile_exhaust: Vec<usize>,
        id_monsters: [Option<usize>; MAX_MONSTERS],
        id_stasis_cards: [Option<usize>; MAX_MONSTERS], // Slot-parallel to `id_monsters`
        id_picked_monster: Option<usize>,
        id_card_last_drawn: Option<usize>,
        id_card_nightmare: Option<usize>,
        id_discover: Vec<usize>,

        // Energy
        energy: Energy,

        // Per-turn counters
        this_turn_discards: u8,
        this_turn_attacks: u8,
        this_turn_cards_played: u8,
        this_turn_panache: u8,

        // Per-combat counters
        this_combat_damage_instances_taken: u8,
        this_combat_escaped: bool,

        // Bomb countdown
        bomb_countdown: u8,

        // Loot staked by the event that started this fight (Mushrooms, Colosseum)
        event_loot: EventLoot,
    },
    CombatEnded,
    Reward {
        reward_id_cards: Vec<Vec<usize>>,
        reward_id_relics: Vec<usize>,
        reward_id_potions: Vec<usize>,
        reward_gold: Option<u16>,
        reward_relics_exclusive: bool, // Wether taking a Relic clears the rest (Boss rewards)
    },
    Event {
        kind: EventKind,
        consumed: bool,
        id_options: Vec<usize>,
    },
    Shop {
        shop_id_cards: Vec<usize>,
        shop_id_relics: Vec<usize>,
        shop_id_potions: Vec<usize>,
        shop_purge_cost: u16,
    },
    Map,
    RestSite,
    Chest,
    ChestOpened,
}

#[derive(Debug, Clone, Copy)]
pub struct Energy {
    pub energy_current: u8,
    pub energy_max: u8,
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

    // Colorless
    Apparition,
    Bite,
    DarkShackles,
    DramaticEntrance,
    Jax,
    Panacea,
    Trip,
    Apotheosis,
    Chrysalis,
    Discovery,
    Enlightenment,
    HandOfGreed,
    Impatience,
    JackOfAllTrades,
    Madness,
    Magnetism,
    Metamorphosis,
    Panache,
    PanicButton,
    SadisticNature,
    ThinkingAhead,
    Transmutation,
    Forethought,
    Mayhem,
    Purity,
    SecretTechnique,
    SecretWeapon,
    TheBomb,
    Violence,
    CurseOfTheBell,
    Wound,
}

// Lifetime of a cost override; Combat writes the base cost and is never stored on the entity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CostScope {
    Turn,
    Combat,
    UntilPlayed,
}

// Destination for Card spawns and moves
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardPile {
    Hand,
    Draw,
    Discard,
    Deck,
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
    Byrd,
    Centurion,
    Chosen,
    Healer,
    Mugger,
    ShelledParasite,
    SnakePlant,
    Snecko,
    SphericGuardian,
    BookOfStabbing,
    GremlinLeader,
    Taskmaster,
    BronzeAutomaton,
    BronzeOrb,
    Champ,
    TheCollector,
    TorchHead,
    BanditBear,
    BanditLeader,
    BanditPointy,
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
    Act2Easy,
    Act2Hard,
    Act2Elite,
    Act2Boss,
    Event,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount)]
#[repr(u8)]
pub enum MonsterEncounter {
    // Act 1 easy
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

    // Event-only (never pooled)
    ThreeFungiBeasts,

    // Act 2 easy
    SphericGuardian,
    Chosen,
    ShelledParasite,
    ThreeByrds,
    TwoThieves,

    // Act 2 hard
    SnakePlant,
    CenturionAndHealer,
    Snecko,
    CultistAndChosen,
    ThreeCultists,
    ShelledParasiteAndFungi,
    ChosenAndByrds,
    SentryAndSphere,

    // Act 2 elite
    GremlinLeader,
    Slavers,
    BookOfStabbing,

    // Act 2 boss
    BronzeAutomaton,
    TheCollector,
    Champ,
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
    WheelOfChange,
    BonfireSpirits,
    OminousForge,
    FaceTrader,
    WeMeetAgain,
    Mushrooms,
    DeadAdventurer,
    Neow,
    Addict,
    Beggar,
    Ghosts,
    BackToBasics,
    Colosseum,
    Designer,
    GremlinMatchGame,
    KnowingSkull,
    MaskedBandits,
    TheJoust,
    TheLibrary,
    TheMausoleum,
    Vampires,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShopSlot {
    Card,
    Relic,
    Potion,
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
    AncientPotion,
    LiquidBronze,
    EssenceOfSteel,
    GhostInAJar,
    CultistPotion,
    CunningPotion,
    DistilledChaos,
    BlessingOfTheForge,
    EntropicBrew,
    RegenerationPotion,
    SteroidPotion,
    SpeedPotion,
    DuplicateNextCardPlayPotion,
    ColorlessPotion,
    GamblersBrew,
    LiquidMemories,
    SneckoOil,
    FairyPotion,
    SmokeBomb,
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
    SpiritPoop,
    WarpedTongs,
    CultistHeadpiece,
    FaceOfCleric,
    NlothsHungryFace,
    SsserpentHead,
    OddMushroom,
    PhilosopherStone,
    CoffeeDripper,
    FusionHammer,
    Sozu,
    CursedKey,
    BustedCrown,
    SlaversCollar,
    Ectoplasm,
    VelvetChoker,
    WristBlade,
    HoveringKite,
    DreamCatcher,
    Cauldron,
    MembershipCard,
    TheCourier,
    GamblingChip,
    BottledFlame,
    BottledLightning,
    BottledTornado,
    Matryoshka,
    Orrery,
    Toolbox,
    SneckoEye,
    Astrolabe,
    CallingBell,
    TinyHouse,
    BlackStar,
    Girya,
    PeacePipe,
    Shovel,
    WingBoots,
    QuestionCard,
    SingingBowl,
    PrayerWheel,
    RunicPyramid,
    RingOfTheSerpent,
    SacredBark,
    NeowsLament,
}

pub fn relic_name_from_u8(v: u8) -> RelicName {
    assert!((v as usize) < RelicName::COUNT, "Invalid RelicName: {v}");
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
