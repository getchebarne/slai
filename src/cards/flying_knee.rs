use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::modifier::ModifierKind;
use crate::cards::Card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static FLYING_KNEE: Card = Card {
    name: CardName::FlyingKnee,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::DamagePhysical {
            base: 8,
            target: TargetKind::CardTarget,
        },
        EffectTemplate::ModifierGain {
            kind: ModifierKind::NextTurnEnergy,
            stacks: 1,
            target: TargetKind::Character,
        },
    ],
};
// Upgraded
pub static FLYING_KNEE_PLUS: Card = Card {
    name: CardName::FlyingKnee,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::DamagePhysical {
            base: 11, // +3 damage
            target: TargetKind::CardTarget,
        },
        EffectTemplate::ModifierGain {
            kind: ModifierKind::NextTurnEnergy,
            stacks: 1,
            target: TargetKind::Character,
        },
    ],
};
