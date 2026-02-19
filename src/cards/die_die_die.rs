use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::cards::Card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static DIE_DIE_DIE: Card = Card {
    name: CardName::DieDieDie,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 1,
    upgraded: false,
    exhaust: true,
    innate: false,
    effects: &[EffectTemplate::DamagePhysical {
        base: 13,
        target: TargetKind::AllMonsters,
    }],
};
// Upgraded
pub static DIE_DIE_DIE_PLUS: Card = Card {
    name: CardName::DieDieDie,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 1,
    upgraded: true,
    exhaust: true,
    innate: false,
    effects: &[EffectTemplate::DamagePhysical {
        base: 17, // + 4 damage
        target: TargetKind::AllMonsters,
    }],
};
