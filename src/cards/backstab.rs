use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::cards::Card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static BACKSTAB: Card = Card {
    name: CardName::Backstab,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 0,
    upgraded: false,
    exhaust: true,
    innate: true,
    effects: &[EffectTemplate::DamagePhysical {
        base: 11,
        target: TargetKind::CardTarget,
    }],
};
// Upgraded
pub static BACKSTAB_PLUS: Card = Card {
    name: CardName::Backstab,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 0,
    upgraded: true,
    exhaust: true,
    innate: true,
    effects: &[EffectTemplate::DamagePhysical {
        base: 15, // +4 damage
        target: TargetKind::CardTarget,
    }],
};
