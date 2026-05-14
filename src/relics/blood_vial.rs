use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

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
