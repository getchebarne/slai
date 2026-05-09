use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_relic};
use crate::types::{RelicName, RelicTier};

pub static ANCHOR: Entity = make_entity_relic(
    RelicName::Anchor,
    RelicTier::Common,
    0,
    &[Effect {
        kind: EffectKind::BlockGain { amount: 10 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::Single,
        },
    }],
);
