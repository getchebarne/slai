use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::relics::RelicTemplate;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_combat_end.rs` (+1 max HP after each combat)
pub static FACE_OF_CLERIC: RelicTemplate = RelicTemplate {
    name: RelicName::FaceOfCleric,
    tier: RelicTier::Special,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[Effect {
        kind: EffectKind::MaxHealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(1),
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
    effects_pickup: &[],
    effects_rest: &[],
    effects_counter: &[],
};
