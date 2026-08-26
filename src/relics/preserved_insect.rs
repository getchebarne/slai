use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Elite enemies start at 75 percent HP
// See:
//    - `process_effect_combat_start.rs`
pub static PRESERVED_INSECT: RelicTemplate = RelicTemplate {
    name: RelicName::PreservedInsect,
    tier: RelicTier::Common,
    counter_init: 0,
    effects_combat_start: &[],
};
