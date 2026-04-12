use crate::cards::Card;
use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static NEUTRALIZE: Card = Card {
    name: CardName::Neutralize,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 0,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: true,
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { base: 3 },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 1,
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
pub static NEUTRALIZE_PLUS: Card = Card {
    name: CardName::Neutralize,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 0,
    upgraded: true,
    exhaust: false,
    innate: false,
    requires_target: true,
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { base: 4 }, // +1 damage
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 2, // +1 stack
            },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
    ],
};
