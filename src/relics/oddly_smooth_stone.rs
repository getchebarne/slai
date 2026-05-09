use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_relic};
use crate::modifier::ModifierKind;
use crate::types::{RelicName, RelicTier};

pub static ODDLY_SMOOTH_STONE: Entity = make_entity_relic(
    RelicName::OddlySmoothStone,
    RelicTier::Common,
    0,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Dexterity,
            stacks: 1,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
