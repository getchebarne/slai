use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static PREPARED: Entity = make_entity_card(
    CardName::Prepared,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    0,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::CardDraw { count: 1 },
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
pub static PREPARED_PLUS: Entity = make_entity_card(
    CardName::Prepared,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    0,
    true,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::CardDraw { count: 2 },
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::CardDiscard,
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Hand,
                selection: SelectionKind::Input { count: 2 },
            },
        },
    ],
);
