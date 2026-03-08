use crate::cards::Card;
use crate::effect::{Candidates, EffectKind, EffectTemplate, SelectionKind, Targeting};
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static CLOAK_AND_DAGGER: Card = Card {
    name: CardName::CloakAndDagger,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[
        EffectTemplate {
            kind: EffectKind::BlockGain { amount: 6 },
            targeting: Some(Targeting {
                candidates: Candidates::Character,
                selection: SelectionKind::All,
            }),
        },
        EffectTemplate {
            kind: EffectKind::AddShivs { count: 1 },
            targeting: None,
        },
    ],
};
// Upgraded
pub static CLOAK_AND_DAGGER_PLUS: Card = Card {
    name: CardName::CloakAndDagger,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[
        EffectTemplate {
            kind: EffectKind::BlockGain { amount: 6 },
            targeting: Some(Targeting {
                candidates: Candidates::Character,
                selection: SelectionKind::All,
            }),
        },
        EffectTemplate {
            kind: EffectKind::AddShivs { count: 2 }, // +1 shiv
            targeting: None,
        },
    ],
};
