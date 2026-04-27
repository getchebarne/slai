use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static SURVIVOR: Entity = make_entity_card(
    CardName::Survivor,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Basic,
    1,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 8 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
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
pub static SURVIVOR_PLUS: Entity = make_entity_card(
    CardName::Survivor,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Basic,
    1,
    true,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::BlockGain {
                amount: 11, // +3 block
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
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
