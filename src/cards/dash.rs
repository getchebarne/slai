use crate::cards::Card;
use crate::effect::{Candidates, EffectKind, EffectTemplate, SelectionKind, Targeting};
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static DASH: Card = Card {
    name: CardName::Dash,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 2,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: true,
    effects: &[
        EffectTemplate {
            kind: EffectKind::BlockGain { amount: 10 },
            targeting: Some(Targeting {
                candidates: Candidates::Character,
                selection: SelectionKind::All,
            }),
        },
        EffectTemplate {
            kind: EffectKind::DamagePhysical { base: 10 },
            targeting: Some(Targeting {
                candidates: Candidates::CardTarget,
                selection: SelectionKind::All,
            }),
        },
    ],
};
// Upgraded
pub static DASH_PLUS: Card = Card {
    name: CardName::Dash,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 2,
    upgraded: true,
    exhaust: false,
    innate: false,
    requires_target: true,
    effects: &[
        EffectTemplate {
            kind: EffectKind::BlockGain {
                amount: 13, // +3 damage
            },
            targeting: Some(Targeting {
                candidates: Candidates::Character,
                selection: SelectionKind::All,
            }),
        },
        EffectTemplate {
            kind: EffectKind::DamagePhysical {
                base: 13, // +3 block
            },
            targeting: Some(Targeting {
                candidates: Candidates::CardTarget,
                selection: SelectionKind::All,
            }),
        },
    ],
};
