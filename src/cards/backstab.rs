use crate::cards::Card;
use crate::effect::{Candidates, EffectKind, EffectTemplate, SelectionKind, Targeting};
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static BACKSTAB: Card = Card {
    name: CardName::Backstab,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 0,
    upgraded: false,
    exhaust: true,
    innate: true,
    requires_target: true,
    effects: &[EffectTemplate {
        kind: EffectKind::DamagePhysical { base: 11 },
        targeting: Some(Targeting {
            candidates: Candidates::CardTarget,
            selection: SelectionKind::All,
        }),
    }],
};
// Upgraded
pub static BACKSTAB_PLUS: Card = Card {
    name: CardName::Backstab,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 0,
    upgraded: true,
    exhaust: true,
    innate: true,
    requires_target: true,
    effects: &[EffectTemplate {
        kind: EffectKind::DamagePhysical { base: 15 }, // +4 damage
        targeting: Some(Targeting {
            candidates: Candidates::CardTarget,
            selection: SelectionKind::All,
        }),
    }],
};
