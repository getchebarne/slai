use crate::cards::Card;
use crate::effect::{Candidates, EffectKind, EffectTemplate, SelectionKind, Targeting};
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static SURVIVOR: Card = Card {
    name: CardName::Survivor,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[
        EffectTemplate {
            kind: EffectKind::BlockGain {
                amount: 8,
                from_card: true,
            },
            targeting: Some(Targeting {
                candidates: Candidates::Character,
                selection: SelectionKind::All,
            }),
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
pub static SURVIVOR_PLUS: Card = Card {
    name: CardName::Survivor,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[
        EffectTemplate {
            kind: EffectKind::BlockGain {
                amount: 11, // +3 block
                from_card: true,
            },
            targeting: Some(Targeting {
                candidates: Candidates::Character,
                selection: SelectionKind::All,
            }),
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
