use crate::cards::Card;
use crate::effect::{EffectKind, EffectTemplate};
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
    requires_target: false,
    effects: &[
        EffectTemplate {
            kind: EffectKind::EnergyGain { amount: 1 },
            targeting: None,
        },
        EffectTemplate {
            kind: EffectKind::CardDraw { count: 2 },
            targeting: None,
        },
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
    requires_target: false,
    effects: &[
        EffectTemplate {
            kind: EffectKind::EnergyGain { amount: 2 }, // +1 energy gain
            targeting: None,
        },
        EffectTemplate {
            kind: EffectKind::CardDraw { count: 2 },
            targeting: None,
        },
    ],
};
