use crate::cards::Card;
use crate::effect::{Candidates, EffectKind, EffectTemplate, SelectionKind, Targeting};
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
    requires_target: true,
    effects: &[
        EffectTemplate {
            kind: EffectKind::DamagePhysical { base: 9 },
            targeting: Some(Targeting {
                candidates: Candidates::CardTarget,
                selection: SelectionKind::All,
            }),
        },
        EffectTemplate {
            kind: EffectKind::CardDraw { count: 1 },
            targeting: None,
        },
        EffectTemplate {
            kind: EffectKind::CardDiscard,
            targeting: Some(Targeting {
                candidates: Candidates::Hand,
                selection: SelectionKind::Input { count: 1 },
            }),
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
    requires_target: true,
    effects: &[
        EffectTemplate {
            kind: EffectKind::DamagePhysical { base: 12 }, // +3 damage
            targeting: Some(Targeting {
                candidates: Candidates::CardTarget,
                selection: SelectionKind::All,
            }),
        },
        EffectTemplate {
            kind: EffectKind::CardDraw { count: 1 },
            targeting: None,
        },
        EffectTemplate {
            kind: EffectKind::CardDiscard,
            targeting: Some(Targeting {
                candidates: Candidates::Hand,
                selection: SelectionKind::Input { count: 1 },
            }),
        },
    ],
};
