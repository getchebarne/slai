use crate::modifier::ModifierKind;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::ChestKind;
use crate::types::DeckSelectKind;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::types::RelicTier;
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
    HealthGain {
        amount: u16,
    },
    HealthLoss {
        amount: u16,
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

    // Master-deck mutation (combat rewards, events, shop, Neow)
    CardRemoveFromDeck,
    CardAddToDeck {
        card_name: CardName,
        upgraded: bool,
    },

    // Out-of-combat HP cap mutation
    MaxHealthGain {
        amount: u16,
    },
    MaxHealthLoss {
        amount: u16,
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
    HealthGainPct {
        numer: u8,
        denom: u8,
    },
    HealthLossPct {
        numer: u8,
        denom: u8,
    },
    MaxHealthLossPct {
        numer: u8,
        denom: u8,
    },
    CardUpgradeRandomInDeck {
        count: u8,
    },
    CardTransformRoll,
    RelicGrantRandom {
        tier: Option<RelicTier>,
    },
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
    DeckSelectStart {
        kind: DeckSelectKind,
    },

    // Halt markers; re-run as `Direct` with the player's pick
    CardDiscoverPick,
    DeckSelectPick {
        kind: DeckSelectKind,
    },
}

// Origin tag the CardDiscard handler branches on
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiscardSource {
    Explicit,
    EndOfTurn,
}

// Source pool for a Resolve effect
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CandidatePool {
    Hand,
    MonsterPicked,
    Character,
    Monsters,
    OtherMonsters,
    Source,
    NextRowRooms,
    IdPick,
    DeckFiltered(DeckSelectKind),
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
    /// Target is known (or not needed). Dispatch runs the handler directly.
    /// `None` means the effect takes no target (CardDraw, EnergyGain, etc.)
    Direct(Option<usize>),

    /// Target must be resolved against live state at dequeue time. The
    /// dispatcher runs `resolve_selection_kind` and either fans out to `Direct`
    /// effects or halts on input
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

pub const fn effect_direct(
    kind: EffectKind,
    id_source: Option<usize>,
    id_target: Option<usize>,
) -> Effect {
    Effect {
        kind,
        id_source,
        target: Target::Direct(id_target),
    }
}

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
