use crate::cards::Card;
use crate::effect::{Candidates, EffectKind, EffectTemplate, SelectionKind, Targeting};
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static BACKFLIP: Card = Card {
    name: CardName::Backflip,
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
            kind: EffectKind::BlockGain {
                amount: 5,
                from_card: true,
            },
            targeting: Some(Targeting {
                candidates: Candidates::Character,
                selection: SelectionKind::All,
            }),
        },
        EffectTemplate {
            kind: EffectKind::CardDraw { count: 2 },
            targeting: None,
        },
    ],
};
// Upgraded
pub static BACKFLIP_PLUS: Card = Card {
    name: CardName::Backflip,
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
            kind: EffectKind::BlockGain {
                amount: 8, // +3 block
                from_card: true,
            },
            targeting: Some(Targeting {
                candidates: Candidates::Character,
                selection: SelectionKind::All,
            }),
        },
        EffectTemplate {
            kind: EffectKind::CardDraw { count: 2 },
            targeting: None,
        },
    ],
};
