use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::modifier::ModifierKind;
use crate::cards::Card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static A_THOUSAND_CUTS: Card = Card {
    name: CardName::AThousandCuts,
    kind: CardKind::Power,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 2,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::ThousandCuts,
        stacks: 1,
        target: TargetKind::Character,
    }],
};
// Upgraded
pub static A_THOUSAND_CUTS_PLUS: Card = Card {
    name: CardName::AThousandCuts,
    kind: CardKind::Power,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 2,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::ThousandCuts,
        stacks: 2, // +1 stack
        target: TargetKind::Character,
    }],
};
