use strum::EnumCount;

use crate::consts::MAX_MONSTERS;
use crate::events::EventKind;

// Vitals: physical combat state. Shared by Character and Monsters
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

// Contexts: persistent GameState fields owning the working memory that is only
// meaningful while `active`; reset in place at entry (buffers retain capacity),
// deactivated at exit. Never reassign a whole context — that drops the buffers
#[derive(Debug, Clone)]
pub struct Combat {
    pub active: bool,
    pub id_card_hand: Vec<usize>,
    pub id_card_draw: Vec<usize>,
    pub id_card_discard: Vec<usize>,
    pub id_card_exhaust: Vec<usize>,
    pub id_monsters: [Option<usize>; MAX_MONSTERS],
    pub id_card_stasis: [Option<usize>; MAX_MONSTERS], // Slot-parallel to `id_monsters`
    pub id_monster_picked: Option<usize>,
    pub id_card_last_drawn: Option<usize>,
    pub id_card_nightmare: Option<usize>,
    pub id_card_discover: Vec<usize>,
    // (combat copy, deck original) pairs; mid-combat spawns have no entry
    pub id_card_origins: Vec<(usize, usize)>,

    // Energy
    pub energy: Energy,

    // Per-turn counters
    pub this_turn_discards: u8,
    pub this_turn_attacks: u8,
    pub this_turn_cards_played: u8,
    pub this_turn_panache: u8,

    // Per-combat counters
    pub this_combat_damage_instances_taken: u8,
    pub this_combat_escaped: bool,

    // Bomb countdown
    pub bomb_countdown: u8,
}

pub fn combat_reset(combat: &mut Combat) {
    combat.id_card_hand.clear();
    combat.id_card_draw.clear();
    combat.id_card_discard.clear();
    combat.id_card_exhaust.clear();
    combat.id_monsters.fill(None);
    combat.id_card_stasis.fill(None);
    combat.id_monster_picked = None;
    combat.id_card_last_drawn = None;
    combat.id_card_nightmare = None;
    combat.id_card_discover.clear();
    combat.id_card_origins.clear();
    combat.energy = Energy {
        energy_current: 0,
        energy_max: 0,
    };
    combat.this_turn_discards = 0;
    combat.this_turn_attacks = 0;
    combat.this_turn_cards_played = 0;
    combat.this_turn_panache = 0;
    combat.this_combat_damage_instances_taken = 0;
    combat.this_combat_escaped = false;
    combat.bomb_countdown = 0;
}

#[derive(Debug, Clone)]
pub struct Reward {
    pub active: bool,
    pub id_cards: Vec<Vec<usize>>,
    pub id_relics: Vec<usize>,
    pub id_potions: Vec<usize>,
    pub gold: Option<u16>,
    pub relics_exclusive: bool, // Wether taking a Relic clears the rest (Boss rewards)
}

pub fn reward_reset(reward: &mut Reward) {
    reward.id_cards.clear();
    reward.id_relics.clear();
    reward.id_potions.clear();
    reward.gold = None;
    reward.relics_exclusive = false;
}

// Find-or-create for the RewardRoll* effects: any roll may activate the context
pub fn reward_ensure(reward: &mut Reward) {
    if !reward.active {
        reward_reset(reward);
        reward.active = true;
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    pub active: bool,
    pub event_kind: EventKind,
    pub consumed: bool,
    pub id_event_options: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Shop {
    pub active: bool,

    // The stock as offers: (entity id, price)
    pub cards: Vec<(usize, u16)>,
    pub relics: Vec<(usize, u16)>,
    pub potions: Vec<(usize, u16)>,
    pub purge_cost: u16,
    pub purged: bool,
}

pub fn shop_reset(shop: &mut Shop) {
    shop.cards.clear();
    shop.relics.clear();
    shop.potions.clear();
    shop.purge_cost = 0;
    shop.purged = false;
}

#[derive(Debug, Clone)]
pub struct RestSite {
    pub active: bool,
    pub consumed: bool,
}

#[derive(Debug, Clone)]
pub struct Chest {
    pub active: bool,
    pub chest_kind: ChestKind,
    pub chest_opened: bool,
}

// The focused context
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Combat,
    Reward,
    Shop,
    Chest,
    RestSite,
    Event,
    Map,
}

#[derive(Debug, Clone, Copy)]
pub struct Energy {
    pub energy_current: u8,
    pub energy_max: u8,
}

pub const VITALS_ZERO: Vitals = Vitals {
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
    RitualDagger,
    Necronomicurse,
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
    KnowingSkull,
    MaskedBandits,
    TheJoust,
    TheLibrary,
    TheMausoleum,
    Vampires,
    Nest,
    CursedTome,
    DrugDealer,
    ForgottenAltar,
    Nloth,
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
    Necronomicon,
    Enchiridion,
    NilrysCodex,
    MutagenicStrength,
    NlothsGift,
    BloodyIdol,
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
