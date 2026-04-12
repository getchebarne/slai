use crate::cards::Card;
use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static ACROBATICS: Card = Card {
    name: CardName::Acrobatics,
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
            kind: EffectKind::CardDraw { count: 3 },
            source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::CardDiscard,
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Hand,
                selection: SelectionKind::Input { count: 1 },
            },
        },
    ],
};
// Upgraded
pub static ACROBATICS_PLUS: Card = Card {
    name: CardName::Acrobatics,
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
            kind: EffectKind::CardDraw { count: 4 }, // +1 draw
            source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::CardDiscard,
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Hand,
                selection: SelectionKind::Input { count: 1 },
            },
        },
    ],
};
