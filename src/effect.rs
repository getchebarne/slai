use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::ChestKind;
use crate::types::CostScope;
use crate::types::DeltaSign;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::types::RewardKind;
use crate::types::RoomKind;

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
    },
    CardAdopt,
    CardCostRandomize,
    CardDiscard {
        source: DiscardSource,
    },
    CardDiscoverPick,
    CardDiscoverRoll {
        kind: Option<CardKind>,
        color: CardColor,
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
    // Relocate an existing combat card to `pile`; not a discard/draw (no triggers)
    CardMove {
        pile: CardPile,
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
    CardTransform,
    CardUpgrade,
    ChestOpen,
    CombatEnd,
    CombatEscape,
    CombatStart {
        event_gold: Option<Amount>,
        event_relic: Option<RelicName>,
        event_relic_roll: bool,
    },
    DamageDeal {
        amount: u16,
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
    LiquidMemoriesPick,
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
    RelicGrantRandom,
    RelicGrantSpecific {
        name: RelicName,
        fallback_circlet: bool,
    },
    RestSiteConsume,
    RewardRollChest {
        kind: ChestKind,
    },
    RewardRollCombat {
        room_kind: RoomKind,
        escaped: bool,
        event_gold: Option<Amount>,
        event_relic: Option<RelicName>,
        event_relic_roll: bool,
    },
    RewardRollPotions {
        count: u8,
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
        scope: CostScope,
    },
    ShopBuild,
    ShopBuyCard,
    ShopBuyPotion,
    ShopBuyRelic,
    ShopPurge,
    ShuffleDiscardPileIntoDrawPile,
    SneakyStrikeProc {
        energy: u8,
    },
    StormOfSteelProc {
        upgraded: bool,
    },
    TargetClear,
    TargetSet,
    TurnEnd,
    TurnStart,
    UnloadDiscard,
    WheelSpin,
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
    Hand { filter: CandidatePoolCardFilter },
    Character,
    Monsters { filter: CandidatePoolMonstersFilter },
    Source,
    Discover,
    Deck { filter: CandidatePoolCardFilter },
    PileDraw { filter: CandidatePoolCardFilter },
    PileDiscard,
    PileExhaust,

    // We Meet Again's rolled picks, read from the event payload at execution time
    EventPickCard,
    EventPickPotion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CandidatePoolCardFilter {
    Purgeable,
    Upgradeable,
    Any,
    Transformable,
    PurgeableCurse,
    Attack,
    Skill,
    Costed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CandidatePoolMonstersFilter {
    All,
    Other,
    Picked,
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
