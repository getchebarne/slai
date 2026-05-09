use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_relic};
use crate::modifier::ModifierKind;
use crate::types::{RelicName, RelicTier};

pub static TWISTED_FUNNEL: Entity = make_entity_relic(
    RelicName::TwistedFunnel,
    RelicTier::Shop,
    0,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Poison,
            stacks: 4,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Monsters,
            selection: SelectionKind::All,
        },
    }],
);
