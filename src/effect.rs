use crate::modifier::ModifierKind;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::ChestKind;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::types::RewardKind;
use crate::types::RoomKind;

// EffectKind: the shared "what happens" enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EffectKind {
    NoOp,
    DamagePhysical {
        amount: u16,
    },
    DamagePhysicalIfPoisoned {
        amount: u16,
    },
    DistractionAdd,
    EscapePlanCheck {
        block: u16,
    },
    GlassKnifeDecay {
        delta: i16,
    },
    FinisherDamage {
        damage: u16,
    },
    FlechettesDamage {
        damage: u16,
    },
    HeelHookProc,
    SneakyStrikeProc {
        energy: u8,
    },
    StormOfSteelProc {
        upgraded: bool,
    },
    UnloadDiscard,
    BlockGain {
        amount: u16,
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
    EnergyGain {
        amount: u16,
    },
    CardDraw {
        count: u16,
    },
    DrawUpTo {
        amount: u8,
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
    CardDiscard {
        source: DiscardSource,
    },
    CardMoveToDiscard,
    DamageMindBlast,
    ShuffleDiscardPileIntoDrawPile,
    CardNightmarePick,
    CardNightmareSpawn,
    CardRetain,
    CardSetupPick,
    SetCostOverride {
        amount: u8,
    },
    CalculatedGamble,

    // Runtime only (for now)
    CardPlay,
    CardExhaust,
    CardRemove,
    CardUpgrade,
    RewardRollCombat {
        room_kind: RoomKind,
    },
    RewardRollChest {
        kind: ChestKind,
    },
    RewardTake {
        kind: RewardKind,
    },
    TargetSet,
    TargetClear,
    DamageDeal {
        amount: u16,
    },
    HealthDelta {
        sign: HealthDeltaSign,
        amount: HealthDeltaAmount,
    },
    BlockSet {
        amount: u16,
    },
    EnergyLoss {
        amount: u8,
    },
    ModifierTick,
    PoisonTick,
    ModifierSetNotNew,
    Death,
    CombatStart,
    CombatEnd,
    TurnStart,
    TurnEnd,
    MoveUpdate,
    MoveExecute,
    RoomEnter,
    RestSiteExit,
    MonsterSpawn {
        name: MonsterName,
    },
    EscapeMonster,
    GoldSteal {
        amount: u8,
    },
    GoldGain {
        amount: u16,
    },
    HexaghostBurnIncrease {
        count: u8,
    },
    HexaghostDivider,

    // Halts on the pick; re-runs as `Direct` once the player chooses
    RoomSelect,

    // Master-deck mutation
    CardPurge,
    CardDuplicate,
    CardTransform,
    CardAddToDeck {
        card_name: CardName,
        upgraded: bool,
    },

    // Out-of-combat HP cap mutation
    MaxHealthDelta {
        sign: HealthDeltaSign,
        amount: HealthDeltaAmount,
    },

    ChestOpen,

    // Potions
    PotionUse,
    PotionAddRandom {
        limited: bool,
    },

    // Umbrella skip: bulk-clear all reward pool fields
    RewardSkip,

    // Rolls N cards of `kind`; halts on CardDiscoverPick for Action::CardDiscoverSelect
    CardDiscoverSelect {
        kind: CardKind,
        count: u8,
    },

    // Event substrate
    GoldLoss {
        amount: u16,
    },
    RelicGrantRandom,
    RelicGrantSpecific {
        name: RelicName,
        fallback_circlet: bool,
    },
    EventAdvanceState {
        delta: i8,
    },
    RollD100Branch {
        chance: u8,
        on_lt: &'static [Effect],
        on_ge: &'static [Effect],
    },
    EventEnd,

    // Halt markers; re-run as `Direct` with the player's pick
    CardDiscoverPick,
}

// Origin tag the CardDiscard handler branches on
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiscardSource {
    Explicit,
    EndOfTurn,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HealthDeltaSign {
    Gain,
    Loss,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HealthDeltaAmount {
    Absolute(u16),
    Relative { numerator: u8, denominator: u8 },
}

// Source pool for a Resolve effect
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CandidatePool {
    Hand,
    Character,
    Monsters { filter: CandidatePoolMonstersFilter },
    Source,
    NextRowRooms,
    IdPick,
    Deck { filter: CandidatePoolDeckFilter },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CandidatePoolDeckFilter {
    Purgeable,
    Upgradeable,
    Any,
    Transformable,
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

// Input count if the Effect's target is a Resolve with SelectionKind::Input
pub fn input_count(effect: &Effect) -> Option<u16> {
    match effect.target {
        Target::Resolve {
            selection_kind: SelectionKind::Input { count },
            ..
        } => Some(count),
        _ => None,
    }
}
