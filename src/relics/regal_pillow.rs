use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::relics::RelicTemplate;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::types::RelicTier;

// Resting heals 15 additional HP
// See:
//    - `action.rs`
pub static REGAL_PILLOW: RelicTemplate = RelicTemplate {
    name: RelicName::RegalPillow,
    tier: RelicTier::Common,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_pickup: &[],
    effects_rest: &[Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(15),
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
    effects_counter: &[],
};
