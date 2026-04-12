use crate::cards::Card;
use crate::effect::{CandidatePool, EffectKind, Effect, SelectionKind, Target};
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static BACKFLIP: Card = Card {
    name: CardName::Backflip,
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
            kind: EffectKind::BlockGain { amount: 5 },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::CardDraw { count: 2 },
            source: None,
            target: Target::Direct(None),
        },
    ],
};
// Upgraded
pub static BACKFLIP_PLUS: Card = Card {
    name: CardName::Backflip,
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
            kind: EffectKind::BlockGain {
                amount: 8, // +3 block
            },
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::CardDraw { count: 2 },
            source: None,
            target: Target::Direct(None),
        },
    ],
};
