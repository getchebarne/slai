use crate::cards::Card;
use crate::effect::{Candidates, EffectKind, EffectTemplate, SelectionKind, Targeting};
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static INFINITE_BLADES: Card = Card {
    name: CardName::InfiniteBlades,
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
            kind: ModifierKind::InfiniteBlades,
            stacks: 1,
        },
        targeting: Some(Targeting {
            candidates: Candidates::Character,
            selection: SelectionKind::All,
        }),
    }],
};
// Upgraded
pub static INFINITE_BLADES_PLUS: Card = Card {
    name: CardName::InfiniteBlades,
    kind: CardKind::Power,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: true, // is innate
    requires_target: false,
    effects: &[EffectTemplate {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::InfiniteBlades,
            stacks: 1,
        },
        targeting: Some(Targeting {
            candidates: Candidates::Character,
            selection: SelectionKind::All,
        }),
    }],
};
