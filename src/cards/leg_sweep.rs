use crate::cards::Card;
use crate::effect::{CandidatePool, EffectKind, Effect, SelectionKind, Target};
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static LEG_SWEEP: Card = Card {
    name: CardName::LegSweep,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 2,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: true,
    effects: &[
        Effect {
            kind: EffectKind::BlockGain { amount: 11 },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 2,
            },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
    ],
};
// Upgraded
pub static LEG_SWEEP_PLUS: Card = Card {
    name: CardName::LegSweep,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 2,
    upgraded: true,
    exhaust: false,
    innate: false,
    requires_target: true,
    effects: &[
        Effect {
            kind: EffectKind::BlockGain {
                amount: 14, // +3 block
            },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 3, // +1 stack
            },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
    ],
};
