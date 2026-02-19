use crate::effect::EffectTemplate;
use crate::cards::Card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static ADRENALINE: Card = Card {
    name: CardName::Adrenaline,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 0,
    upgraded: false,
    exhaust: true,
    innate: false,
    effects: &[
        EffectTemplate::EnergyGain { amount: 1 },
        EffectTemplate::CardDraw { count: 2 },
    ],
};
// Upgraded
pub static ADRENALINE_PLUS: Card = Card {
    name: CardName::Adrenaline,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 0,
    upgraded: true,
    exhaust: true,
    innate: false,
    effects: &[
        EffectTemplate::EnergyGain { amount: 2 }, // +1 energy gain
        EffectTemplate::CardDraw { count: 2 },
    ],
};
