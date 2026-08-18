use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// The first HP loss each combat draws 3 Cards
// See:
//    - `process_effect_health_delta.rs`
pub static CENTENNIAL_PUZZLE: RelicTemplate = RelicTemplate {
    name: RelicName::CentennialPuzzle,
    tier: RelicTier::Common,
    counter_init: 0,
    effects_combat_start: &[],
};
