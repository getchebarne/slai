use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::modifier::ModifierKind;
use crate::cards::Card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static AFTER_IMAGE: Card = Card {
    name: CardName::AfterImage,
    kind: CardKind::Power,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::AfterImage,
        stacks: 1,
        target: TargetKind::Character,
    }],
};
// Upgraded
pub static AFTER_IMAGE_PLUS: Card = Card {
    name: CardName::AfterImage,
    kind: CardKind::Power,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: true, // is innate
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::AfterImage,
        stacks: 1,
        target: TargetKind::Character,
    }],
};
