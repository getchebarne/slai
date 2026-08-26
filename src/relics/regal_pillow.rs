use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Resting heals 15 additional HP
// See:
//    - `action.rs`
pub static REGAL_PILLOW: RelicTemplate = RelicTemplate {
    name: RelicName::RegalPillow,
    tier: RelicTier::Common,
    counter_init: 0,
    effects_combat_start: &[],
};
