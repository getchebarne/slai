use crate::cards::Card;
use crate::effect::{Candidates, EffectKind, EffectTemplate, SelectionKind, Targeting};
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static PHANTASMAL_KILLER: Card = Card {
    name: CardName::PhantasmalKiller,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[EffectTemplate {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Phantasmal,
            stacks: 1,
        },
        targeting: Some(Targeting {
            candidates: Candidates::Character,
            selection: SelectionKind::All,
        }),
    }],
};
// Upgraded
pub static PHANTASMAL_KILLER_PLUS: Card = Card {
    name: CardName::PhantasmalKiller,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 0, // -1 cost
    upgraded: true,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[EffectTemplate {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Phantasmal,
            stacks: 1,
        },
        targeting: Some(Targeting {
            candidates: Candidates::Character,
            selection: SelectionKind::All,
        }),
    }],
};
