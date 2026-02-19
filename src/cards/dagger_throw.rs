use crate::effect::EffectTemplate;
use crate::effect::SelectionKind;
use crate::effect::TargetKind;
use crate::cards::Card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static DAGGER_THROW: Card = Card {
    name: CardName::DaggerThrow,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::DamagePhysical {
            base: 9,
            target: TargetKind::CardTarget,
        },
        EffectTemplate::CardDraw { count: 1 },
        EffectTemplate::CardDiscard {
            selection: SelectionKind::Input,
        },
    ],
};
// Upgraded
pub static DAGGER_THROW_PLUS: Card = Card {
    name: CardName::DaggerThrow,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::DamagePhysical {
            base: 12, // +3 damage
            target: TargetKind::CardTarget,
        },
        EffectTemplate::CardDraw { count: 1 },
        EffectTemplate::CardDiscard {
            selection: SelectionKind::Input,
        },
    ],
};
