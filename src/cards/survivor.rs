use crate::cards::Card;
use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static SURVIVOR: Card = Card {
    name: CardName::Survivor,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 8,
            target: TargetKind::Character,
        },
        EffectTemplate::CardDiscardInput,
    ],
};
// Upgraded
pub static SURVIVOR_PLUS: Card = Card {
    name: CardName::Survivor,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 11, // +3 block
            target: TargetKind::Character,
        },
        EffectTemplate::CardDiscardInput,
    ],
};
