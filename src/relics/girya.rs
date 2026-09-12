use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Lift at rest sites (max 3); combats open with Strength equal to lifts
// See:
//    - `action.rs`
//    - `process_effect_girya_lift.rs`
//    - `process_effect_combat_start.rs`
pub static GIRYA: RelicTemplate = RelicTemplate {
    name: RelicName::Girya,
    tier: RelicTier::Rare,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    effects_on_rest: &[],
    effects_counter: &[],
};
