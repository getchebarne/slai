use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::modifier::ModifierKind;
use crate::cards::Card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static DODGE_AND_ROLL: Card = Card {
    name: CardName::DodgeAndRoll,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 4,
            target: TargetKind::Character,
        },
        EffectTemplate::ModifierGain {
            kind: ModifierKind::NextTurnBlock,
            stacks: 4,
            target: TargetKind::Character,
        },
    ],
};
// Upgraded
pub static DODGE_AND_ROLL_PLUS: Card = Card {
    name: CardName::DodgeAndRoll,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 6, // +2 block
            target: TargetKind::Character,
        },
        EffectTemplate::ModifierGain {
            kind: ModifierKind::NextTurnBlock,
            stacks: 6, // +2 next-turn-block
            target: TargetKind::Character,
        },
    ],
};
