use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::modifier::ModifierKind;
use crate::cards::Card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static ACCURACY: Card = Card {
    name: CardName::Accuracy,
    kind: CardKind::Power,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::Accuracy,
        stacks: 4,
        target: TargetKind::Character,
    }],
};
// Upgraded
pub static ACCURACY_PLUS: Card = Card {
    name: CardName::Accuracy,
    kind: CardKind::Power,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::Accuracy,
        stacks: 6, // +2 stacks
        target: TargetKind::Character,
    }],
};
