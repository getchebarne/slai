use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, choose a deck Card and obtain a copy of it
// See:
//    - `process_effect_relic_adopt.rs`
pub static DOLLYS_MIRROR: RelicTemplate = RelicTemplate {
    name: RelicName::DollysMirror,
    tier: RelicTier::Shop,
    counter_init: 0,
    effects_combat_start: &[],
};
