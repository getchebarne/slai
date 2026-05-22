use crate::modifier::ModifierKind;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::ChestKind;
use crate::types::DeckSelectKind;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::types::RelicTier;
use crate::types::RoomKind;

// EffectKind: the shared "what happens" enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EffectKind {
    Noop,
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
    CardRewardClear,
    RewardRollCombat {
        room_kind: RoomKind,
    },
    RewardRollChest {
        kind: ChestKind,
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

    // Select: halts the queue asking the player to pick a target. After the
    // pick, the same EffectKind runs as `Direct` with the chosen entity,
    // mutating state and pushing follow-up effects
    RoomSelect,

    // Relic flow
    RewardTakeRelic,

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
    RewardTakePotion,

    // Gold reward pickup (in-pool gold from combat-end or chest)
    RewardTakeGold,

    // Umbrella skip: bulk-clear all reward pool fields
    RewardSkip,

    // Roll N random cards of `kind`; halt on a CardDiscoverPick at queue head.
    // Player picks one via `Action::CardDiscoverSelect`
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

    // Queue-head halt markers. Resolved via Target::Resolve { _, Input{count} };
    // the Direct form applies the pick once the player has chosen
    CardDiscoverPick,
    DeckSelectPick {
        kind: DeckSelectKind,
    },
}

// DiscardSource: tags a CardDiscard effect with its origin so the handler can branch on it
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiscardSource {
    Explicit,
    EndOfTurn,
}

// CandidatePool: abstract source pool for a Resolve effect's target resolution
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

// Default-zero Effect, used to fill the fixed-size Entity.card_effects array.
// Slots past `card_effects_len` are ignored
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

// True iff the Effect's target requires player input. Sound between
// process_queue calls only -- inside the loop, resolve_targets is authoritative
// since Input{count} auto-resolves when count >= candidate count
pub fn effect_awaits_input(effect: &Effect) -> bool {
    matches!(
        effect.target,
        Target::Resolve {
            selection: SelectionKind::Input { .. },
            ..
        }
    )
}
