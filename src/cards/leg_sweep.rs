use crate::cards::Card;
use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static LEG_SWEEP: Card = Card {
    name: CardName::LegSweep,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 2,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 11,
            target: TargetKind::Character,
        },
        EffectTemplate::ModifierGain {
            kind: ModifierKind::Weak,
            stacks: 2,
            target: TargetKind::CardTarget,
        },
    ],
};
// Upgraded
pub static LEG_SWEEP_PLUS: Card = Card {
    name: CardName::LegSweep,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 2,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 14, // +3 block
            target: TargetKind::Character,
        },
        EffectTemplate::ModifierGain {
            kind: ModifierKind::Weak,
            stacks: 3, // +1 stack
            target: TargetKind::CardTarget,
        },
    ],
};
