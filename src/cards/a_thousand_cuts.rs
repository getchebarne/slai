use crate::cards::Card;
use crate::effect::{Candidates, EffectKind, EffectTemplate, SelectionKind, Targeting};
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static A_THOUSAND_CUTS: Card = Card {
    name: CardName::AThousandCuts,
    kind: CardKind::Power,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 2,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[EffectTemplate {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::ThousandCuts,
            stacks: 1,
        },
        targeting: Some(Targeting {
            candidates: Candidates::Character,
            selection: SelectionKind::All,
        }),
    }],
};
// Upgraded
pub static A_THOUSAND_CUTS_PLUS: Card = Card {
    name: CardName::AThousandCuts,
    kind: CardKind::Power,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 2,
    upgraded: true,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[EffectTemplate {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::ThousandCuts,
            stacks: 2, // +1 stack
        },
        targeting: Some(Targeting {
            candidates: Candidates::Character,
            selection: SelectionKind::All,
        }),
    }],
};
