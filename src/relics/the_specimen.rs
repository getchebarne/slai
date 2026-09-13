use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// An enemy dying with Poison passes it to a random enemy
// See:
//    - `process_effect_death.rs`
pub static THE_SPECIMEN: RelicTemplate = RelicTemplate {
    name: RelicName::TheSpecimen,
    tier: RelicTier::Rare,
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
