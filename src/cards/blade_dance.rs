use crate::effect::EffectTemplate;
use crate::cards::Card;
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
    effects: &[EffectTemplate::AddShivs { count: 3 }],
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
    effects: &[EffectTemplate::AddShivs { count: 4 }], // +1 shiv
};
