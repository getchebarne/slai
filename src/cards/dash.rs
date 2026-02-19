use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::cards::Card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static DASH: Card = Card {
    name: CardName::Dash,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 2,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 10,
            target: TargetKind::Character,
        },
        EffectTemplate::DamagePhysical {
            base: 10,
            target: TargetKind::CardTarget,
        },
    ],
};
// Upgraded
pub static DASH_PLUS: Card = Card {
    name: CardName::Dash,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 2,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 13, // +3 damage
            target: TargetKind::Character,
        },
        EffectTemplate::DamagePhysical {
            base: 13, // +3 block
            target: TargetKind::CardTarget,
        },
    ],
};
