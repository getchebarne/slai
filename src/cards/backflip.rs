use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::cards::Card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static BACKFLIP: Card = Card {
    name: CardName::Backflip,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 5,
            target: TargetKind::Character,
        },
        EffectTemplate::CardDraw { count: 2 },
    ],
};
// Upgraded
pub static BACKFLIP_PLUS: Card = Card {
    name: CardName::Backflip,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 8, // +3 block
            target: TargetKind::Character,
        },
        EffectTemplate::CardDraw { count: 2 },
    ],
};
