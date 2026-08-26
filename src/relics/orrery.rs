use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On purchase, stages 5 Card bundles in the shop
// See:
//    - `process_effect_relic_adopt.rs`
pub static ORRERY: RelicTemplate = RelicTemplate {
    name: RelicName::Orrery,
    tier: RelicTier::Shop,
    counter_init: 0,
    effects_combat_start: &[],
};
