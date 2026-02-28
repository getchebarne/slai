// Effect system: runtime effects + card/monster-level effect templates.

use crate::modifier::ModifierKind;
use crate::types::EntityId;

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
    CardDiscardInput,
    CardDiscardRandom,
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
        card_id: EntityId,
    },
    CardDiscard {
        card_id: EntityId,
    },
    CardDiscardAll,
    CardExhaust {
        card_id: EntityId,
    },
    CardRemove {
        card_id: EntityId,
    },
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
        target: EntityId,
    },
    TargetClear,

    // Damage
    DamagePhysical {
        source: EntityId,
        target: EntityId,
        base: u16,
    },
    DamageDeal {
        target: EntityId,
        amount: u16,
    },

    // Vitals
    HealthGain {
        target: EntityId,
        amount: u16,
    },
    HealthLoss {
        target: EntityId,
        amount: u16,
    },
    BlockGain {
        target: EntityId,
        amount: u16,
        from_card: bool,
    },
    BlockSet {
        target: EntityId,
        amount: u16,
    },
    EnergyGain {
        amount: u8,
    },
    EnergyLoss {
        amount: u8,
    },

    // Modifiers
    ModifierGain {
        target: EntityId,
        kind: ModifierKind,
        stacks: i16,
    },
    ModifierRemove {
        target: EntityId,
        kind: ModifierKind,
    },
    ModifierTick {
        target: EntityId,
    },
    ModifierSetNotNew,

    // Lifecycle
    Death {
        actor: EntityId,
    },
    CombatStart,
    CombatEnd,
    TurnStart {
        actor: EntityId,
    },
    TurnEnd {
        actor: EntityId,
    },
    MoveUpdate {
        monster: EntityId,
    },
    RoomEnter,
    GameEnd,

    // Await input (pause the queue)
    AwaitMapNode,
    AwaitCardReward,
    AwaitDiscard,
}
