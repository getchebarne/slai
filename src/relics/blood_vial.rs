use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::relics::RelicTemplate;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_combat_start.rs`
pub static BLOOD_VIAL: RelicTemplate = RelicTemplate {
    name: RelicName::BloodVial,
    tier: RelicTier::Common,
    counter_init: 0,
    effects_combat_start: &[Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(2),
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
};
