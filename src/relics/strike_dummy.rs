use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Strike-tagged Cards deal 3 more damage
// See:
//    - `process_effect_damage_physical.rs`
pub static STRIKE_DUMMY: RelicTemplate = RelicTemplate {
    name: RelicName::StrikeDummy,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    effects_combat_start: &[],
};
