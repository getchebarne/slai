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
    effects_combat_start: &[],
};
