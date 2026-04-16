use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static CLOAK_AND_DAGGER: Entity = make_entity_card(
    CardName::CloakAndDagger, CardKind::Skill, CardColor::Green, CardRarity::Common,
    1, false, false, false, false,
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 6 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::AddShivs { count: 1 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
);
// Upgraded
pub static CLOAK_AND_DAGGER_PLUS: Entity = make_entity_card(
    CardName::CloakAndDagger, CardKind::Skill, CardColor::Green, CardRarity::Common,
    1, true, false, false, false,
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 6 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::AddShivs { count: 2 }, // +1 shiv
            id_source: None,
            target: Target::Direct(None),
        },
    ],
);
