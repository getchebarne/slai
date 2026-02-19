use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::modifier::ModifierKind;
use crate::cards::Card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static TERROR: Card = Card {
    name: CardName::Terror,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: false,
    exhaust: true,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::Vulnerable,
        stacks: 99,
        target: TargetKind::CardTarget,
    }],
};
// Upgraded
pub static TERROR_PLUS: Card = Card {
    name: CardName::Terror,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 0, // -1 cost
    upgraded: true,
    exhaust: true,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::Vulnerable,
        stacks: 99,
        target: TargetKind::CardTarget,
    }],
};
