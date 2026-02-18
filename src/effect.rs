// Effect system: runtime effects + card/monster-level effect templates.

use crate::modifier::ModifierKind;
use crate::types::ActorId;

// ---------------------------------------------------------------------------
// EffectTemplate: stored on Card and Move, used for RL encoding + instantiation
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TargetKind {
    CardTarget,
    Character,
    AllMonsters,
    Source,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionKind {
    Input,
    Random,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EffectTemplate {
    DamagePhysical {
        base: u16,
        target: TargetKind,
    },
    BlockGain {
        amount: u16,
        target: TargetKind,
    },
    ModifierGain {
        kind: ModifierKind,
        stacks: i16,
        target: TargetKind,
    },
    ModifierRemove {
        kind: ModifierKind,
        target: TargetKind,
    },
    EnergyGain {
        amount: u8,
    },
    AddShivs {
        count: u8,
    },
    CardDraw {
        count: u8,
    },
    CardDiscard {
        selection: SelectionKind,
    },
    CalculatedGamble,
}

// ---------------------------------------------------------------------------
// Effect: runtime effect queued during gameplay
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Effect {
    // Card operations
    CardDraw {
        count: u8,
    },
    CardPlay {
        card_idx: usize,
    },
    CardDiscard {
        card_idx: usize,
    },
    CardDiscardAll,
    CardExhaust {
        card_idx: usize,
    },
    CardRemove {
        card_idx: usize,
    },
    CardActiveSet {
        card_idx: usize,
    },
    CardActiveClear,
    AddShivs {
        count: u8,
    },
    CalculatedGamble,
    CardUpgrade {
        deck_idx: usize,
    },

    // Card rewards
    CardRewardRoll,
    CardRewardSelect {
        reward_idx: usize,
    },
    CardRewardClear,

    // Targeting
    TargetSet {
        monster_idx: u8,
    },
    TargetClear,

    // Damage
    DamagePhysical {
        source: ActorId,
        target: ActorId,
        base: u16,
    },
    DamageDeal {
        target: ActorId,
        amount: u16,
    },

    // Vitals
    HealthGain {
        target: ActorId,
        amount: u16,
    },
    HealthLoss {
        target: ActorId,
        amount: u16,
    },
    BlockGain {
        target: ActorId,
        amount: u16,
        from_card: bool,
    },
    BlockSet {
        target: ActorId,
        amount: u16,
    },
    EnergyGain {
        amount: u8,
    },
    EnergyLoss {
        amount: u8,
    },

    // Modifiers (one variant, not 17)
    ModifierGain {
        target: ActorId,
        kind: ModifierKind,
        stacks: i16,
    },
    ModifierRemove {
        target: ActorId,
        kind: ModifierKind,
    },
    ModifierTick {
        target: ActorId,
    },
    ModifierSetNotNew,

    // Lifecycle
    Death {
        actor: ActorId,
    },
    CombatStart,
    CombatEnd,
    TurnStart {
        actor: ActorId,
    },
    TurnEnd {
        actor: ActorId,
    },
    MonsterMoveUpdate {
        monster_idx: u8,
    },
    RoomEnter,
    GameEnd,

    // Await input (pause the queue)
    AwaitMapNode,
    AwaitCardReward,
    AwaitDiscard,
}
