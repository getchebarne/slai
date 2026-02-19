use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::modifier::ModifierKind;
use crate::cards::Card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static BURST: Card = Card {
    name: CardName::Burst,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::Burst,
        stacks: 1,
        target: TargetKind::Character,
    }],
};
// Upgraded
pub static BURST_PLUS: Card = Card {
    name: CardName::Burst,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::Burst,
        stacks: 2, // +1 stack
        target: TargetKind::Character,
    }],
};
