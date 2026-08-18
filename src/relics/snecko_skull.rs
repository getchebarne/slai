use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Applying Poison to an enemy applies 1 more
// See:
//    - `process_effect_modifier_gain.rs`
pub static SNECKO_SKULL: RelicTemplate = RelicTemplate {
    name: RelicName::SneckoSkull,
    tier: RelicTier::Common,
    counter_init: 0,
    effects_combat_start: &[],
};
