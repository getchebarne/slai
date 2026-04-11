use crate::cards::Card;
use crate::effect::{CandidatePool, EffectKind, Effect, SelectionKind, Targeting};
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static STRIKE: Card = Card {
    name: CardName::Strike,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: true,
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { base: 6 },
        source: None,
        targeting: Targeting::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
};
// Upgraded
pub static STRIKE_PLUS: Card = Card {
    name: CardName::Strike,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    requires_target: true,
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { base: 9 }, // +3 damage
        source: None,
        targeting: Targeting::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::All,
        },
    }],
};
