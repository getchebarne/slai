use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::cards::Card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static SHIV: Card = Card {
    name: CardName::Shiv,
    kind: CardKind::Attack,
    color: CardColor::Colorless,
    rarity: CardRarity::Special,
    cost: 0,
    upgraded: false,
    exhaust: true,
    innate: false,
    effects: &[EffectTemplate::DamagePhysical {
        base: 4,
        target: TargetKind::CardTarget,
    }],
};
// Upgraded
pub static SHIV_PLUS: Card = Card {
    name: CardName::Shiv,
    kind: CardKind::Attack,
    color: CardColor::Colorless,
    rarity: CardRarity::Special,
    cost: 0,
    upgraded: true,
    exhaust: true,
    innate: false,
    effects: &[EffectTemplate::DamagePhysical {
        base: 6, // +2 damage
        target: TargetKind::CardTarget,
    }],
};
