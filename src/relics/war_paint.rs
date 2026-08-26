use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, upgrade 2 random Skills
// See:
//    - `process_effect_relic_adopt.rs`
pub static WAR_PAINT: RelicTemplate = RelicTemplate {
    name: RelicName::WarPaint,
    tier: RelicTier::Common,
    counter_init: 0,
    effects_combat_start: &[],
};
