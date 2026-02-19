use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::cards::Card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static DEFLECT: Card = Card {
    name: CardName::Deflect,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 0,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::BlockGain {
        amount: 4,
        target: TargetKind::Character,
    }],
};
// Upgraded
pub static DEFLECT_PLUS: Card = Card {
    name: CardName::Deflect,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 0,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::BlockGain {
        amount: 7, // +3 block
        target: TargetKind::Character,
    }],
};
