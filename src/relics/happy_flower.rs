use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::relics::RelicTemplate;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 3rd turn start grants 1 energy; counter persists across combats
// See:
//    - `process_effect_turn_start.rs`
pub static HAPPY_FLOWER: RelicTemplate = RelicTemplate {
    name: RelicName::HappyFlower,
    tier: RelicTier::Common,
    counter_init: 0,
    counter_reset: 3,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    effects_on_rest: &[],
    effects_counter: &[Effect {
        kind: EffectKind::EnergyDelta {
            sign: DeltaSign::Gain,
            amount: 1,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
};
