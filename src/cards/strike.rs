use crate::cards::Card;
use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static STRIKE: Card = Card {
    name: CardName::Strike,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::DamagePhysical {
        base: 6,
        target: TargetKind::CardTarget,
    }],
};
// Upgraded
pub static STRIKE_PLUS: Card = Card {
    name: CardName::Strike,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::DamagePhysical {
        base: 9, // +3 damage
        target: TargetKind::CardTarget,
    }],
};
