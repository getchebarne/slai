use crate::modifier::ModifierKind;

// EffectKind: the shared "what happens" enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EffectKind {
    DamagePhysical { base: u16 },
    BlockGain { amount: u16 },
    ModifierGain { kind: ModifierKind, stacks: i16 },
    ModifierRemove { kind: ModifierKind },
    EnergyGain { amount: u8 },
    AddShivs { count: u8 },
    CardDraw { count: u8 },
    CardDiscard,
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
    ModifierSetNotNew,
    Death,
    CombatStart,
    CombatEnd,
    TurnStart,
    TurnEnd,
    MoveUpdate,
    RoomEnter,
    RestSiteExit,

    // Halt-kind variants — pending player decisions.
    // Each one's dispatch arm returns ProcessEffectResult::Halt { phase_new }.
    MapNodeSelect,
    CardRewardSelect,
    GameOver,
    AwaitCombatAction,
    AwaitRestSiteAction,
    AwaitCardReward,
}

// CandidatePool: abstract source pool for a Resolve effect's target resolution.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CandidatePool {
    Hand,
    CardTarget,
    Character,
    Monsters,
    Source,
    MapNodeNextRow,
    CardRewardPool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionKind {
    All,
    Random { count: u8 },
    Input { count: u8 },
}

// Target: whether an Effect's target is already known (Direct) or must be
// resolved against live state when the effect is dequeued (Resolve).
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
// runtime-synthesized effects (which use `Direct` target).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Effect {
    pub kind: EffectKind,
    pub source: Option<usize>,
    pub target: Target,
}

impl Effect {
    /// Constructs an `Effect` with `Direct` target. Convenience for the
    /// common case where a runtime-synthesized effect already knows its target.
    pub const fn direct(
        kind: EffectKind,
        source: Option<usize>,
        target: Option<usize>,
    ) -> Self {
        Self {
            kind,
            source,
            target: Target::Direct(target),
        }
    }
}
