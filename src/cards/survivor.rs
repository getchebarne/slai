use crate::cards::Card;
use crate::effect::{CandidatePool, EffectKind, Effect, SelectionKind, Targeting};
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static SURVIVOR: Card = Card {
    name: CardName::Survivor,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[
        Effect {
            kind: EffectKind::BlockGain { amount: 8 },
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
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
pub static SURVIVOR_PLUS: Card = Card {
    name: CardName::Survivor,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[
        Effect {
            kind: EffectKind::BlockGain {
                amount: 11, // +3 block
            },
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
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
