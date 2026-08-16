use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::CardRarity;
use crate::types::CostScope;
use crate::types::DeltaSign;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::types::RelicTier;
use crate::types::RewardKind;
use crate::types::ShopSlot;

// EffectKind: the shared "what happens" enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EffectKind {
    ActTransition,
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
    CardAdopt,
    CardBottle,
    CardDiscard {
        source: DiscardSource,
    },
    CardDiscoverPick {
        cost_zero: Option<CostScope>,
        pile: CardPile,
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
    CombatStart,
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
    DebuffsClear,
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
    JoustBet {
        on_owner: bool,
    },
    KnowingSkullAsk {
        wish: KnowingSkullWish,
    },
    MausoleumOpen,
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
        minion: bool, // Gremlin Leader's summons
        // Skip the spawn when `cap` of this name are already rostered (Torch Heads)
        cap: Option<u8>,
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
    RelicGrantPool {
        pool: &'static [RelicName],
    },
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
    RitualDaggerProc {
        bump: u16,
    },
    RewardRollCards {
        bundles: u8,
        rare_only: bool,
    },
    RewardRollGold {
        amount: Amount,
    },
    RewardRollLibraryCards,
    RewardRollNeowCards {
        colorless: bool,
        rare_only: bool,
    },
    RewardRollPotion {
        eligible: bool,
    },
    RewardRollPotions {
        count: u8,
        uniform: bool,
    },
    RewardRollRelic {
        pick: RelicPick,
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
    StasisSteal,
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

// Knowing Skull's escalating asks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KnowingSkullWish {
    Potion,
    Gold,
    Card,
}

// How far the staged relic is already resolved; each variant rolls only what remains
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RelicPick {
    Thresholds { th_common: u8, th_uncommon: u8 },
    Tier(RelicTier),
    Name(RelicName),
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
    NotMinion,

    // Starter-card predicates (Vampires, Back to Basics)
    StarterStrike,
    StarterUpgradeable,
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
pub const EFFECT_ZERO: Effect = Effect {
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

// Discover pick: choose 1 of the rolled Cards; cost break and destination vary by caller
pub const fn effect_discover_pick(cost_zero: Option<CostScope>, pile: CardPile) -> Effect {
    Effect {
        kind: EffectKind::CardDiscoverPick { cost_zero, pile },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Discover,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::Input { count: 1 },
        },
    }
}

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
