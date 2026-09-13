use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// A Card reward may be skipped for +2 max HP (no heal)
// See:
//    - `action.rs`
//    - `process_effect_singing_bowl_proc.rs`
pub static SINGING_BOWL: RelicTemplate = RelicTemplate {
    name: RelicName::SingingBowl,
    tier: RelicTier::Uncommon,
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
