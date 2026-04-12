use crate::cards::Card;
use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static BURST: Card = Card {
    name: CardName::Burst,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Burst,
            stacks: 1,
        },
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
};
// Upgraded
pub static BURST_PLUS: Card = Card {
    name: CardName::Burst,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Burst,
            stacks: 2, // +1 stack
        },
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
};
