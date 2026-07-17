use crate::modifier::ModifierKind;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::ChestKind;
use crate::types::DeltaSign;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::types::RewardKind;
use crate::types::RoomKind;

// EffectKind: the shared "what happens" enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EffectKind {
    BlockGain {
        amount: u16,
    },
    BlockSet {
        amount: u16,
    },
    BonfireOffer,
    CalculatedGamble,
    CardAddToDeck {
        card_name: CardName,
        upgraded: bool,
    },
    CardAddToDiscard {
        card_name: CardName,
        count: u8,
        upgraded: bool,
    },
    CardAddToHand {
        card_name: CardName,
        count: u16,
        upgraded: bool,
    },
    CardAdopt,
    CardDiscard {
        source: DiscardSource,
    },
    CardDiscoverPick,
    CardDiscoverRoll {
        kind: CardKind,
        count: u8,
    },
    CardDraw {
        count: u16,
    },
    CardDrawUpTo {
        amount: u8,
    },
    CardDuplicate,
    CardExhaust,
    CardMoveToDiscard,
    CardNightmarePick,
    CardNightmareSpawn,
    CardPlay,
    CardPurge,
    CardRemove,
    CardRetain,
    CardSetupPick,
    CardTransform,
    CardUpgrade,
    ChestOpen,
    CombatEnd,
    CombatStart,
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
    EnergyGain {
        amount: u16,
    },
    EnergyLoss {
        amount: u8,
    },
    EscapePlanCheck {
        block: u16,
    },
    EventAdvanceState {
        delta: i8,
    },
    EventConsume,
    FaceTrade,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CandidatePoolCardFilter {
    Purgeable,
    Upgradeable,
    Any,
    Transformable,
    PurgeableCurse,
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
