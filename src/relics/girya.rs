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
    effects_combat_start: &[],
};
