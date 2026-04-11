use crate::cards::Card;
use crate::effect::{CandidatePool, EffectKind, Effect, SelectionKind, Targeting};
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
