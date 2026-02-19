use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::modifier::ModifierKind;
use crate::cards::Card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static FOOTWORK: Card = Card {
    name: CardName::Footwork,
    kind: CardKind::Power,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::Dexterity,
        stacks: 2,
        target: TargetKind::Character,
    }],
};
// Upgraded
pub static FOOTWORK_PLUS: Card = Card {
    name: CardName::Footwork,
    kind: CardKind::Power,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::Dexterity,
        stacks: 3, // +1 dexterity
        target: TargetKind::Character,
    }],
};
