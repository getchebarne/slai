use crate::cards::Card;
use crate::effect::{Candidates, EffectKind, EffectTemplate, SelectionKind, Targeting};
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static FLYING_KNEE: Card = Card {
    name: CardName::FlyingKnee,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: true,
    effects: &[
        EffectTemplate {
            kind: EffectKind::DamagePhysical { base: 8 },
            targeting: Some(Targeting {
                candidates: Candidates::CardTarget,
                selection: SelectionKind::All,
            }),
        },
        EffectTemplate {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::NextTurnEnergy,
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
pub static FLYING_KNEE_PLUS: Card = Card {
    name: CardName::FlyingKnee,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    requires_target: true,
    effects: &[
        EffectTemplate {
            kind: EffectKind::DamagePhysical {
                base: 11, // +3 damage
            },
            targeting: Some(Targeting {
                candidates: Candidates::CardTarget,
                selection: SelectionKind::All,
            }),
        },
        EffectTemplate {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::NextTurnEnergy,
                stacks: 1,
            },
            targeting: Some(Targeting {
                candidates: Candidates::Character,
                selection: SelectionKind::All,
            }),
        },
    ],
};
