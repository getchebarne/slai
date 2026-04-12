use crate::cards::Card;
use crate::effect::{CandidatePool, EffectKind, Effect, SelectionKind, Target};
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static DASH: Card = Card {
    name: CardName::Dash,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 2,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: true,
    effects: &[
        Effect {
            kind: EffectKind::BlockGain { amount: 10 },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { base: 10 },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
    ],
};
// Upgraded
pub static DASH_PLUS: Card = Card {
    name: CardName::Dash,
    kind: CardKind::Attack,
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
                amount: 13, // +3 damage
            },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical {
                base: 13, // +3 block
            },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
    ],
};
