use crate::cards::Card;
use crate::effect::{Candidates, EffectKind, EffectTemplate, SelectionKind, Targeting};
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static ACCURACY: Card = Card {
    name: CardName::Accuracy,
    kind: CardKind::Power,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[EffectTemplate {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Accuracy,
            stacks: 4,
        },
        targeting: Some(Targeting {
            candidates: Candidates::Character,
            selection: SelectionKind::All,
        }),
    }],
};
// Upgraded
pub static ACCURACY_PLUS: Card = Card {
    name: CardName::Accuracy,
    kind: CardKind::Power,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[EffectTemplate {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Accuracy,
            stacks: 6, // +2 stacks
        },
        targeting: Some(Targeting {
            candidates: Candidates::Character,
            selection: SelectionKind::All,
        }),
    }],
};
