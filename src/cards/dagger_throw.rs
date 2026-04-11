use crate::cards::Card;
use crate::effect::{CandidatePool, EffectKind, Effect, SelectionKind, Targeting};
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static DAGGER_THROW: Card = Card {
    name: CardName::DaggerThrow,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: true,
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { base: 9 },
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::CardDraw { count: 1 },
            source: None,
            targeting: Targeting::Direct(None),
        },
        Effect {
            kind: EffectKind::CardDiscard,
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Hand,
                selection: SelectionKind::Input { count: 1 },
            },
        },
    ],
};
// Upgraded
pub static DAGGER_THROW_PLUS: Card = Card {
    name: CardName::DaggerThrow,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    requires_target: true,
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { base: 12 }, // +3 damage
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::CardDraw { count: 1 },
            source: None,
            targeting: Targeting::Direct(None),
        },
        Effect {
            kind: EffectKind::CardDiscard,
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Hand,
                selection: SelectionKind::Input { count: 1 },
            },
        },
    ],
};
