use crate::cards::Card;
use crate::effect::{Candidates, EffectKind, EffectTemplate, SelectionKind, Targeting};
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static DIE_DIE_DIE: Card = Card {
    name: CardName::DieDieDie,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 1,
    upgraded: false,
    exhaust: true,
    innate: false,
    requires_target: false,
    effects: &[EffectTemplate {
        kind: EffectKind::DamagePhysical { base: 13 },
        targeting: Some(Targeting {
            candidates: Candidates::Monsters,
            selection: SelectionKind::All,
        }),
    }],
};
// Upgraded
pub static DIE_DIE_DIE_PLUS: Card = Card {
    name: CardName::DieDieDie,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 1,
    upgraded: true,
    exhaust: true,
    innate: false,
    requires_target: false,
    effects: &[EffectTemplate {
        kind: EffectKind::DamagePhysical {
            base: 17, // +4 damage
        },
        targeting: Some(Targeting {
            candidates: Candidates::Monsters,
            selection: SelectionKind::All,
        }),
    }],
};
