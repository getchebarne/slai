use crate::modifier::ModifierKind;
use crate::types::{CardName, MonsterName};

// EffectKind: the shared "what happens" enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EffectKind {
    Noop,
    DamagePhysical { amount: u16 },
    DamagePhysicalIfPoisoned { amount: u16 },
    DistractionAdd,
    EscapePlanCheck { block: u16 },
    GlassKnifeDecay { delta: i16 },
    FinisherDamage { damage: u16 },
    FlechettesDamage { damage: u16 },
    HeelHookProc,
    SneakyStrikeProc { energy: u8 },
    StormOfSteelProc { upgraded: bool },
    UnloadDiscard,
    BlockGain { amount: u16 },
    ModifierGain { kind: ModifierKind, stacks: i16 },
    ModifierMultiply { kind: ModifierKind, factor: u8 },
    ModifierRemove { kind: ModifierKind },
    EnergyGain { amount: u8 },
    CardDraw { count: u8 },
    DrawUpTo { target: u8 },
    CardAddToDiscard {
        card_name: CardName,
        count: u8,
        upgraded: bool,
    },
    CardAddToHand {
        card_name: CardName,
        count: u8,
        upgraded: bool,
    },
    CardDiscard { source: DiscardSource },
    CardMoveToDiscard,
    CardNightmarePick,
    CardNightmareSpawn,
    CardRetain,
    CardSetupPick,
    SetCostOverride { amount: u8 },
    CalculatedGamble,

    // Runtime only (for now)
    CardPlay,
    CardExhaust,
    CardRemove,
    CardUpgrade,
    CardRewardRoll,
    CardRewardClear,
    TargetSet,
    TargetClear,
    DamageDeal { amount: u16 },
    HealthGain { amount: u16 },
    HealthLoss { amount: u16 },
    BlockSet { amount: u16 },
    EnergyLoss { amount: u8 },
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
    MonsterSpawn { name: MonsterName },
    EscapeMonster,
    GoldSteal { amount: u8 },
    GoldGain { amount: u16 },
    HexaghostBurnIncrease { count: u8 },
    HexaghostDivider,

    // Select: halts the queue asking the player to pick a target. After the
    // pick, the same EffectKind runs as `Direct` with the chosen entity,
    // mutating state and pushing follow-up effects
    RoomSelect,
    CardRewardSelect,

    // Relic flow
    RelicRewardRoll,
    RelicRewardSelect,
    RelicRewardClear,

    // Master-deck mutation (events, shop purge, Neow). Resolved against id_deck
    DeckCardRemove,
    CardAddSpecific { card_name: CardName, upgraded: bool },
    CurseAdd,

    // Out-of-combat HP cap mutation
    MaxHpAdd { amount: u16 },
    MaxHpSub { amount: u16 },
}

// DiscardSource: tags a CardDiscard effect with its origin so the handler can
// branch on it. Explicit = card- or player-driven discard (counter bumps,
// fires `card_on_discard_effects`); EndOfTurn = turn-end auto-discard
// (honors `card_retain` and `card_ethereal`, no counter, no triggers)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiscardSource {
    Explicit,
    EndOfTurn,
}

// CandidatePool: abstract source pool for a Resolve effect's target resolution
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CandidatePool {
    Hand,
    CardTarget,
    Character,
    Monsters,
    OtherMonsters,
    Source,
    NextRowRooms,
    CardRewardPool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionKind {
    All,
    Single,
    Random { count: u8 },
    Input { count: u8 },
}

// Target: whether an Effect's target is already known (Direct) or must be
// resolved against live state when the effect is dequeued (Resolve)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Target {
    /// Target is known (or not needed). Dispatch runs the handler directly.
    /// `None` means the effect takes no target (CardDraw, EnergyGain, etc.).
    Direct(Option<usize>),

    /// Target must be resolved against live state at dequeue time. The
    /// dispatcher runs `resolve_targets` and either fans out to `Direct`
    /// effects or halts on input.
    Resolve {
        candidates: CandidatePool,
        selection: SelectionKind,
    },
}

// Effect: a unit of work in the queue. Unified type used for both static card
// and monster-move definitions (which use `Resolve` target) and
// runtime-synthesized effects (which use `Direct` target)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Effect {
    pub kind: EffectKind,
    pub id_source: Option<usize>,
    pub target: Target,
}

// Default-zero Effect, used to fill fixed-size arrays (Entity.card_effects,
// EffectBuf, etc.). Slots past `*_len` are ignored
pub const ZERO_EFFECT: Effect = Effect {
    kind: EffectKind::Noop,
    id_source: None,
    target: Target::Direct(None),
};

impl Effect {
    pub const fn direct(
        kind: EffectKind,
        id_source: Option<usize>,
        id_target: Option<usize>,
    ) -> Self {
        Self {
            kind,
            id_source,
            target: Target::Direct(id_target),
        }
    }
}
