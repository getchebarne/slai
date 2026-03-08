use crate::cards::Card;
use crate::effect::{Candidates, EffectKind, EffectTemplate, SelectionKind, Targeting};
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static BLUR: Card = Card {
    name: CardName::Blur,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
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
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Blur,
                stacks: 1,
            },
            targeting: Some(Targeting {
                candidates: Candidates::Character,
                selection: SelectionKind::All,
            }),
        },
    ],
};
// Upgraded
pub static BLUR_PLUS: Card = Card {
    name: CardName::Blur,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
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
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Blur,
                stacks: 1,
            },
            targeting: Some(Targeting {
                candidates: Candidates::Character,
                selection: SelectionKind::All,
            }),
        },
    ],
};
