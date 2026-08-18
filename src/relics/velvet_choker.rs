use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// +1 energy; no more than 6 Cards can be played per turn
// See:
//    - `process_effect_combat_start.rs`
//    - `action.rs`
pub static VELVET_CHOKER: RelicTemplate = RelicTemplate {
    name: RelicName::VelvetChoker,
    tier: RelicTier::Boss,
    counter_init: 0,
    effects_combat_start: &[],
};
