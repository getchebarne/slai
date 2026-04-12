use crate::cards::Card;
use crate::effect::{CandidatePool, EffectKind, Effect, SelectionKind, Target};
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static SHIV: Card = Card {
    name: CardName::Shiv,
    kind: CardKind::Attack,
    color: CardColor::Colorless,
    rarity: CardRarity::Special,
    cost: 0,
    upgraded: false,
    exhaust: true,
    innate: false,
    requires_target: true,
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { base: 4 },
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
};
// Upgraded
pub static SHIV_PLUS: Card = Card {
    name: CardName::Shiv,
    kind: CardKind::Attack,
    color: CardColor::Colorless,
    rarity: CardRarity::Special,
    cost: 0,
    upgraded: true,
    exhaust: true,
    innate: false,
    requires_target: true,
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { base: 6 }, // +2 damage
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
};
