use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// +1 energy; gold can no longer be gained
// See:
//    - `process_effect_combat_start.rs`
//    - `process_effect_gold_delta.rs`
//    - `process_effect_combat_end.rs`
pub static ECTOPLASM: RelicTemplate = RelicTemplate {
    name: RelicName::Ectoplasm,
    tier: RelicTier::Boss,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_pickup: &[],
    effects_rest: &[],
    effects_counter: &[],
};
