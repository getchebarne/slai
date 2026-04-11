use crate::cards::Card;
use crate::effect::{CandidatePool, EffectKind, Effect, SelectionKind, Targeting};
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static BACKSTAB: Card = Card {
    name: CardName::Backstab,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 0,
    upgraded: false,
    exhaust: true,
    innate: true,
    requires_target: true,
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { base: 11 },
        source: None,
        targeting: Targeting::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
};
// Upgraded
pub static BACKSTAB_PLUS: Card = Card {
    name: CardName::Backstab,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 0,
    upgraded: true,
    exhaust: true,
    innate: true,
    requires_target: true,
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { base: 15 }, // +4 damage
        source: None,
        targeting: Targeting::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
};
