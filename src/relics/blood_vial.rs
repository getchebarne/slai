use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_combat_start.rs`
pub static BLOOD_VIAL: Entity = make_entity_relic(
    RelicName::BloodVial,
    RelicTier::Common,
    0,
    &[Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(2),
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
);
