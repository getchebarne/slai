use crate::cards::Card;
use crate::effect::{CandidatePool, EffectKind, Effect, SelectionKind, Targeting};
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static CLOAK_AND_DAGGER: Card = Card {
    name: CardName::CloakAndDagger,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[
        Effect {
            kind: EffectKind::BlockGain { amount: 6 },
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::AddShivs { count: 1 },
            source: None,
            targeting: Targeting::Direct(None),
        },
    ],
};
// Upgraded
pub static CLOAK_AND_DAGGER_PLUS: Card = Card {
    name: CardName::CloakAndDagger,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[
        Effect {
            kind: EffectKind::BlockGain { amount: 6 },
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::AddShivs { count: 2 }, // +1 shiv
            source: None,
            targeting: Targeting::Direct(None),
        },
    ],
};
