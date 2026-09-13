use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Elite combats start with 2 Strength
// See:
//    - `process_effect_combat_start.rs`
pub static SLING_OF_COURAGE: RelicTemplate = RelicTemplate {
    name: RelicName::SlingOfCourage,
    tier: RelicTier::Shop,
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
