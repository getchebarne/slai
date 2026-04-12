use crate::cards::Card;
use crate::effect::{Effect, EffectKind, Target};
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static CALCULATED_GAMBLE: Card = Card {
    name: CardName::CalculatedGamble,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 0,
    upgraded: false,
    exhaust: true,
    innate: false,
    requires_target: false,
    effects: &[Effect {
        kind: EffectKind::CalculatedGamble,
        source: None,
        target: Target::Direct(None),
    }],
};
// Upgraded
pub static CALCULATED_GAMBLE_PLUS: Card = Card {
    name: CardName::CalculatedGamble,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 0,
    upgraded: true,
    exhaust: false, // doesn't exhaust
    innate: false,
    requires_target: false,
    effects: &[Effect {
        kind: EffectKind::CalculatedGamble,
        source: None,
        target: Target::Direct(None),
    }],
};
