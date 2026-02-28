use crate::cards::Card;
use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static INFINITE_BLADES: Card = Card {
    name: CardName::InfiniteBlades,
    kind: CardKind::Power,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::InfiniteBlades,
        stacks: 1,
        target: TargetKind::Character,
    }],
};
// Upgraded
pub static INFINITE_BLADES_PLUS: Card = Card {
    name: CardName::InfiniteBlades,
    kind: CardKind::Power,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: true, // is innate
    requires_target: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::InfiniteBlades,
        stacks: 1,
        target: TargetKind::Character,
    }],
};
