use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// The Character can no longer become Frail
// See:
//    - `process_effect_modifier_gain.rs`
pub static TURNIP: RelicTemplate = RelicTemplate {
    name: RelicName::Turnip,
    tier: RelicTier::Rare,
    counter_init: 0,
    effects_combat_start: &[],
};
