use crate::cards::Card;
use crate::effect::{Candidates, EffectKind, EffectTemplate, SelectionKind, Targeting};
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static STRIKE: Card = Card {
    name: CardName::Strike,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: true,
    effects: &[EffectTemplate {
        kind: EffectKind::DamagePhysical { base: 6 },
        targeting: Some(Targeting {
            candidates: Candidates::CardTarget,
            selection: SelectionKind::All,
        }),
    }],
};
// Upgraded
pub static STRIKE_PLUS: Card = Card {
    name: CardName::Strike,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    requires_target: true,
    effects: &[EffectTemplate {
        kind: EffectKind::DamagePhysical { base: 9 }, // +3 damage
        targeting: Some(Targeting {
            candidates: Candidates::CardTarget,
            selection: SelectionKind::All,
        }),
    }],
};
