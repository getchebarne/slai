use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_relic};
use crate::modifier::ModifierKind;
use crate::types::{RelicName, RelicTier};

pub static BAG_OF_MARBLES: Entity = make_entity_relic(
    RelicName::BagOfMarbles,
    RelicTier::Common,
    0,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Vulnerable,
            stacks: 1,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Monsters,
            selection: SelectionKind::All,
        },
    }],
);
