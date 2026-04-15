use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, card_entity};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static ACROBATICS: Entity = card_entity(
    CardName::Acrobatics, CardKind::Skill, CardColor::Green, CardRarity::Common,
    1, false, false, false, false,
    &[
        Effect {
            kind: EffectKind::CardDraw { count: 3 },
            source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::CardDiscard,
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Hand,
                selection: SelectionKind::Input { count: 1 },
            },
        },
    ],
);
// Upgraded
pub static ACROBATICS_PLUS: Entity = card_entity(
    CardName::Acrobatics, CardKind::Skill, CardColor::Green, CardRarity::Common,
    1, true, false, false, false,
    &[
        Effect {
            kind: EffectKind::CardDraw { count: 4 }, // +1 draw
            source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::CardDiscard,
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Hand,
                selection: SelectionKind::Input { count: 1 },
            },
        },
    ],
);
