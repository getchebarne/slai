use crate::cards::Card;
use crate::effect::{Candidates, EffectKind, EffectTemplate, SelectionKind, Targeting};
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static SHIV: Card = Card {
    name: CardName::Shiv,
    kind: CardKind::Attack,
    color: CardColor::Colorless,
    rarity: CardRarity::Special,
    cost: 0,
    upgraded: false,
    exhaust: true,
    innate: false,
    requires_target: true,
    effects: &[EffectTemplate {
        kind: EffectKind::DamagePhysical { base: 4 },
        targeting: Some(Targeting {
            candidates: Candidates::CardTarget,
            selection: SelectionKind::All,
        }),
    }],
};
// Upgraded
pub static SHIV_PLUS: Card = Card {
    name: CardName::Shiv,
    kind: CardKind::Attack,
    color: CardColor::Colorless,
    rarity: CardRarity::Special,
    cost: 0,
    upgraded: true,
    exhaust: true,
    innate: false,
    requires_target: true,
    effects: &[EffectTemplate {
        kind: EffectKind::DamagePhysical { base: 6 }, // +2 damage
        targeting: Some(Targeting {
            candidates: Candidates::CardTarget,
            selection: SelectionKind::All,
        }),
    }],
};
