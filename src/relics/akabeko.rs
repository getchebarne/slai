use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::modifier::ModifierKind;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_combat_start.rs`
pub static AKABEKO: Entity = make_entity_relic(
    RelicName::Akabeko,
    RelicTier::Common,
    0,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Vigor,
            stacks: 8,
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    }],
);
