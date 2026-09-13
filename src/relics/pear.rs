use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::relics::RelicTemplate;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, gain 10 max HP
// See:
//    - `process_effect_relic_adopt.rs`
pub static PEAR: RelicTemplate = RelicTemplate {
    name: RelicName::Pear,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_pickup: &[Effect {
        kind: EffectKind::MaxHealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(10),
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
    effects_rest: &[],
    effects_counter: &[],
};
