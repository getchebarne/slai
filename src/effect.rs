use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::CardRarity;
use crate::types::ChestKind;
use crate::types::CostScope;
use crate::types::DeltaSign;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::types::RelicTier;
use crate::types::RewardKind;
use crate::types::RoomKind;
use crate::types::ShopSlot;

// EffectKind: the shared "what happens" enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EffectKind {
    AdventurerSearch,
    BlockGain {
        amount: u16,
    },
    BlockSet {
        amount: u16,
    },
    BonfireOffer,
    CardAdd {
        card_name: CardName,
        pile: CardPile,
        count: u16,
        upgraded: bool,
    },
    CardAddRandom {
        color: CardColor,
        kind: Option<CardKind>,
        pile: CardPile,
        count: u8,
        cost_zero: Option<CostScope>,
        upgraded: bool,
        rarity: Option<CardRarity>,
    },
    CardAddToDeck,
    CardBottle,
    CardDiscard {
        source: DiscardSource,
    },
    CardDiscoverPick {
        cost_zero: Option<CostScope>,
    },
    CardDiscoverRoll {
        kind: Option<CardKind>,
        color: CardColor,
        exclude: &'static [CardName],
        count: u8,
    },
    CardDraw {
        count: u16,
    },
    CardDrawIfNoAttacks {
        count: u16,
    },
    CardDrawUpTo {
        amount: u8,
    },
    CardDuplicate,
    CardExhaust,
    CardMove {
        pile: CardPile,
        cost_zero: Option<CostScope>,
    },
    CardNightmarePick,
    CardNightmareSpawn,
    CardPlay,
    CardPlayFromDrawTop,
    CardPurge,
    CardRemove,
    CardRetain,
    CardSetupPick {
        free: bool,
        bottom: bool,
    },
    CardTransform {
        upgraded: bool,
    },
    CardUpgrade,
    ChestOpen,
    CombatEnd {
        escaped_character: bool,
    },
    CombatStart {
        event_gold: Option<Amount>,
        event_relic: Option<RelicName>,
        event_relic_roll: bool,
    },
    DamageDeal {
        amount: u16,
        lifesteal: bool, // Life Suck
    },
    DamageFinisher {
        damage: u16,
    },
    DamageFlechettes {
        damage: u16,
    },
    DamageMindBlast,
    DamagePhysical {
        amount: u16,
        lifesteal: bool, // Life Suck
    },
    DamagePhysicalIfPoisoned {
        amount: u16,
    },
    Death,
    DistractionAdd,
    EnergyDelta {
        sign: DeltaSign,
        amount: u16,
    },
    EscapePlanCheck {
        block: u16,
    },
    EventAdvanceState {
        delta: i8,
    },
    EventConsume,
    FaceTrade,
    Gamble {
        choose_discards: bool,
        discards_before: Option<u8>,
    },
    GiryaLift,
    GlassKnifeDecay {
        delta: i16,
    },
    GoldDelta {
        sign: DeltaSign,
        amount: Amount,
    },
    GoldSteal {
        amount: u8,
    },
    GremlinSummon,
    HandOfGreedProc {
        gold: u16,
    },
    HealthDelta {
        sign: DeltaSign,
        amount: Amount,
    },
    HealthSet {
        amount: Amount,
    },
    HeelHookProc,
    HexaghostBurnIncrease {
        count: u8,
    },
    MaxHealthDelta {
        sign: DeltaSign,
        amount: Amount,
    },
    ModifierGain {
        kind: ModifierKind,
        stacks: i16,
    },
    ModifierMultiply {
        kind: ModifierKind,
        factor: u8,
    },
    ModifierRemove {
        kind: ModifierKind,
    },
    ModifierSetNotNew,
    ModifierTick,
    MonsterEscape,
    MonsterSpawn {
        name: MonsterName,
    },
    MonsterSplit {
        name: MonsterName,
    },
    MoveExecute,
    MoveUpdate {
        move_override: Option<usize>,
    },
    NoOp,
    PoisonTick,
    PotionAddRandom {
        limited: bool,
    },
    PotionAdopt,
    PotionDiscard,
    PotionUse,
    RelicAdopt,
    RelicGrantRandom {
        tier: Option<RelicTier>,
    },
    RelicGrantSpecific {
        name: RelicName,
        fallback_circlet: bool,
    },
    RelicLose {
        name: RelicName,
    },
    RestSiteConsume,
    RewardRoll {
        source: RewardSource,
    },
    RewardTake {
        kind: RewardKind,
    },
    RoomEnter,
    RoomExit,
    RoomSelect,
    ScrapOozeReach {
        dmg: u16,
        chance: u8,
        advance_on_miss: bool,
    },
    SetCostOverride {
        amount: u8,
        only_reduce: bool,
        random: bool,
        scope: CostScope,
    },
    ShopBuild,
    ShopBuy {
        slot: ShopSlot,
    },
    ShopPurge,
    ShuffleDiscardPileIntoDrawPile,
    SingingBowlProc {
        idx_bundle: u8,
    },
    SneakyStrikeProc {
        energy: u8,
    },
    StormOfSteelProc {
        upgraded: bool,
    },
    StrengthLoseTemp {
        stacks: i16,
    },
    TargetClear,
    TargetSet,
    TurnEnd,
    TurnStart,
    UnloadDiscard,
    WheelSpin,
}

// What a reward roll is rolling for; the handler branches on it
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RewardSource {
    Cards {
        bundles: usize,
    },
    Chest {
        kind: ChestKind,
    },
    Combat {
        room_kind: RoomKind,
        escaped: bool,
        event_gold: Option<Amount>,
        event_relic: Option<RelicName>,
        event_relic_roll: bool,
    },

    // Neow's card offers: always 3, Neow-specific rarity rules
    NeowCards {
        colorless: bool,
        rare_only: bool,
    },
    Potions {
        count: u8,
        uniform: bool,
    },
}

// Origin tag the CardDiscard handler branches on
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiscardSource {
    Explicit,
    EndOfTurn,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Amount {
    Absolute(u16),
    Relative { numerator: u8, denominator: u8 }, // Truncated
    RelativeRounded { numerator: u8, denominator: u8 }, // Rounded half-up instead of truncated
    RelativeCeil { numerator: u8, denominator: u8 }, // Rounded up instead of truncated
    Range { min: u16, max: u16 },

    // We Meet Again's rolled ask, read from the event payload at execution time
    // TODO: revisit, ugly
    EventGoldAsk,
}

// Source pool for a Resolve effect
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CandidatePool {
    Hand,
    Character,
    Monsters,
    Source,
    Discover,
    Deck,
    PileDraw,
    PileDiscard,
    PileExhaust,

    // We Meet Again's rolled picks, read from the event payload at execution time
    EventPickCard,
    EventPickPotion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CandidateFilter {
    // Compare against `Entity` fields
    Any,
    Purgeable,
    Upgradeable,
    Transformable,
    PurgeableCurse,
    KindAttack,
    KindSkill,
    KindPower,
    Costed,

    // Compare against the `Target::Resolve` context
    Picked,
    NotSource,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionKind {
    All,
    Single,
    Random { count: u8 },
    Input { count: u16 },
    InputUpTo { count: u16 },
}

// Target known at queue time (Direct) or resolved against live state at dequeue (Resolve)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Target {
    // Known target, or `None` for targetless effects (CardDraw, EnergyGain).
    Direct(Option<usize>),

    // Resolved against live state at dequeue via `resolve_selection_kind`.
    Resolve {
        candidate_pool: CandidatePool,
        filter: CandidateFilter,
        selection_kind: SelectionKind,
    },
}

// A unit of work in the queue; static defs use `Resolve`, runtime uses `Direct`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Effect {
    pub kind: EffectKind,
    pub id_source: Option<usize>,
    pub target: Target,
}

// Filler for slots past `card_effects_len` in Entity.card_effects
pub const ZERO_EFFECT: Effect = Effect {
    kind: EffectKind::NoOp,
    id_source: None,
    target: Target::Direct(None),
};

// The Resolve shapes static defs use almost everywhere
pub const TARGET_CHARACTER: Target = Target::Resolve {
    candidate_pool: CandidatePool::Character,
    filter: CandidateFilter::Any,
    selection_kind: SelectionKind::Single,
};

pub const TARGET_MONSTER_PICKED: Target = Target::Resolve {
    candidate_pool: CandidatePool::Monsters,
    filter: CandidateFilter::Picked,
    selection_kind: SelectionKind::Single,
};

pub const TARGET_SOURCE: Target = Target::Resolve {
    candidate_pool: CandidatePool::Source,
    filter: CandidateFilter::Any,
    selection_kind: SelectionKind::Single,
};

pub const TARGET_MONSTERS_ALL: Target = Target::Resolve {
    candidate_pool: CandidatePool::Monsters,
    filter: CandidateFilter::Any,
    selection_kind: SelectionKind::All,
};
