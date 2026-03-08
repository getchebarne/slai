use crate::modifier::ModifierKind;
use crate::types::EntityId;

// ---------------------------------------------------------------------------
// EffectKind: the shared "what happens" enum, used by both templates and runtime
// ---------------------------------------------------------------------------

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
    CardDiscardAll,
    CardUpgrade { deck_idx: usize },
    CardRewardRoll,
    CardRewardSelect { idx_reward: usize },
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
    GameEnd,
    AwaitMapNode,
    AwaitCardReward,
    AwaitDiscard,
}

// ---------------------------------------------------------------------------
// Targeting: abstract targeting for card/monster effect definitions
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Candidates {
    Hand,
    CardTarget,
    Character,
    Monsters,
    Source,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionKind {
    All,
    Random { count: u8 },
    Input { count: u8 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Targeting {
    pub candidates: Candidates,
    pub selection: SelectionKind,
}

// ---------------------------------------------------------------------------
// EffectTemplate: card/monster effect definitions (abstract targeting)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EffectTemplate {
    pub kind: EffectKind,
    pub targeting: Option<Targeting>,
}

// ---------------------------------------------------------------------------
// Effect: runtime effect queued during gameplay (resolved entity IDs)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Effect {
    pub kind: EffectKind,
    pub source: Option<EntityId>,
    pub target: Option<EntityId>,
}
