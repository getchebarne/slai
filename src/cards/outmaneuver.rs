use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::modifier::ModifierKind;
use crate::cards::Card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static OUTMANEUVER: Card = Card {
    name: CardName::Outmaneuver,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::NextTurnEnergy,
        stacks: 2,
        target: TargetKind::Character,
    }],
};
// Upgraded
pub static OUTMANEUVER_PLUS: Card = Card {
    name: CardName::Outmaneuver,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::NextTurnEnergy,
        stacks: 3, // +1 next-turn-energy
        target: TargetKind::Character,
    }],
};
