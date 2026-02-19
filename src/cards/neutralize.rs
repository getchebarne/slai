use crate::cards::Card;
use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static NEUTRALIZE: Card = Card {
    name: CardName::Neutralize,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 0,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::DamagePhysical {
            base: 3,
            target: TargetKind::CardTarget,
        },
        EffectTemplate::ModifierGain {
            kind: ModifierKind::Weak,
            stacks: 1,
            target: TargetKind::CardTarget,
        },
    ],
};
// Upgraded
pub static NEUTRALIZE_PLUS: Card = Card {
    name: CardName::Neutralize,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 0,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::DamagePhysical {
            base: 4, // +1 damage
            target: TargetKind::CardTarget,
        },
        EffectTemplate::ModifierGain {
            kind: ModifierKind::Weak,
            stacks: 2, // +1 stack
            target: TargetKind::CardTarget,
        },
    ],
};
