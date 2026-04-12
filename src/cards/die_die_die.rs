use crate::cards::Card;
use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static DIE_DIE_DIE: Card = Card {
    name: CardName::DieDieDie,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 1,
    upgraded: false,
    exhaust: true,
    innate: false,
    requires_target: false,
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { base: 13 },
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Monsters,
            selection: SelectionKind::All,
        },
    }],
};
// Upgraded
pub static DIE_DIE_DIE_PLUS: Card = Card {
    name: CardName::DieDieDie,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 1,
    upgraded: true,
    exhaust: true,
    innate: false,
    requires_target: false,
    effects: &[Effect {
        kind: EffectKind::DamagePhysical {
            base: 17, // +4 damage
        },
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Monsters,
            selection: SelectionKind::All,
        },
    }],
};
