use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, gain 7 max HP
// See:
//    - `process_effect_relic_adopt.rs`
pub static STRAWBERRY: RelicTemplate = RelicTemplate {
    name: RelicName::Strawberry,
    tier: RelicTier::Common,
    counter_init: 0,
    effects_combat_start: &[],
};
