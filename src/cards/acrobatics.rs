use crate::effect::EffectTemplate;
use crate::effect::SelectionKind;
use crate::cards::Card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static ACROBATICS: Card = Card {
    name: CardName::Acrobatics,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::CardDraw { count: 3 },
        EffectTemplate::CardDiscard {
            selection: SelectionKind::Input,
        },
    ],
};
// Upgraded
pub static ACROBATICS_PLUS: Card = Card {
    name: CardName::Acrobatics,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::CardDraw { count: 4 }, // +1 draw
        EffectTemplate::CardDiscard {
            selection: SelectionKind::Input,
        },
    ],
};
