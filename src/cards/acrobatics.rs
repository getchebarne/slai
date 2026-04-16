use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static ACROBATICS: Entity = make_entity_card(
    CardName::Acrobatics, CardKind::Skill, CardColor::Green, CardRarity::Common,
    1, false, false, false, false,
    &[
        Effect {
            kind: EffectKind::CardDraw { count: 3 },
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::CardDiscard,
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Hand,
                selection: SelectionKind::Input { count: 1 },
            },
        },
    ],
);
// Upgraded
pub static ACROBATICS_PLUS: Entity = make_entity_card(
    CardName::Acrobatics, CardKind::Skill, CardColor::Green, CardRarity::Common,
    1, true, false, false, false,
    &[
        Effect {
            kind: EffectKind::CardDraw { count: 4 }, // +1 draw
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::CardDiscard,
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Hand,
                selection: SelectionKind::Input { count: 1 },
            },
        },
    ],
);
