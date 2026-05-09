use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_relic};
use crate::types::{RelicName, RelicTier};

pub static BLOOD_VIAL: Entity = make_entity_relic(
    RelicName::BloodVial,
    RelicTier::Common,
    0,
    &[Effect {
        kind: EffectKind::HealthGain { amount: 2 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::Single,
        },
    }],
);
