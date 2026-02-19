use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::modifier::ModifierKind;
use crate::cards::Card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static PHANTASMAL_KILLER: Card = Card {
    name: CardName::PhantasmalKiller,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::Phantasmal,
        stacks: 1,
        target: TargetKind::Character,
    }],
};
// Upgraded
pub static PHANTASMAL_KILLER_PLUS: Card = Card {
    name: CardName::PhantasmalKiller,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 0, // -1 cost
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::Phantasmal,
        stacks: 1,
        target: TargetKind::Character,
    }],
};
