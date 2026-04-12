use crate::cards::Card;
use crate::effect::{Effect, EffectKind, Target};
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static BLADE_DANCE: Card = Card {
    name: CardName::BladeDance,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[Effect {
        kind: EffectKind::AddShivs { count: 3 },
        source: None,
        target: Target::Direct(None),
    }],
};
// Upgraded
pub static BLADE_DANCE_PLUS: Card = Card {
    name: CardName::BladeDance,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[Effect {
        kind: EffectKind::AddShivs { count: 4 }, // +1 shiv
        source: None,
        target: Target::Direct(None),
    }],
};
