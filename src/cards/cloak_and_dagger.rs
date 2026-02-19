use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::cards::Card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static CLOAK_AND_DAGGER: Card = Card {
    name: CardName::CloakAndDagger,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 6,
            target: TargetKind::Character,
        },
        EffectTemplate::AddShivs { count: 1 },
    ],
};
// Upgraded
pub static CLOAK_AND_DAGGER_PLUS: Card = Card {
    name: CardName::CloakAndDagger,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 6,
            target: TargetKind::Character,
        },
        EffectTemplate::AddShivs { count: 2 }, // +1 shiv
    ],
};
