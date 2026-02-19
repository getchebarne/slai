use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::modifier::ModifierKind;
use crate::cards::Card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static BLUR: Card = Card {
    name: CardName::Blur,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 5,
            target: TargetKind::Character,
        },
        EffectTemplate::ModifierGain {
            kind: ModifierKind::Blur,
            stacks: 1,
            target: TargetKind::Character,
        },
    ],
};
// Upgraded
pub static BLUR_PLUS: Card = Card {
    name: CardName::Blur,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 8, // +3 block
            target: TargetKind::Character,
        },
        EffectTemplate::ModifierGain {
            kind: ModifierKind::Blur,
            stacks: 1,
            target: TargetKind::Character,
        },
    ],
};
