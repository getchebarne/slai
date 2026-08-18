use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, gain 10 max HP
// See:
//    - `process_effect_relic_adopt.rs`
pub static PEAR: RelicTemplate = RelicTemplate {
    name: RelicName::Pear,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    effects_combat_start: &[],
};
