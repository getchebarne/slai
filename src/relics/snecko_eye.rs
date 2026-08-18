use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Draw 2 additional Cards each turn; drawn Cards get a random cost (0-3)
// See:
//    - `process_effect_turn_start.rs`
//    - `process_effect_card_draw.rs`
pub static SNECKO_EYE: RelicTemplate = RelicTemplate {
    name: RelicName::SneckoEye,
    tier: RelicTier::Boss,
    counter_init: 0,
    effects_combat_start: &[],
};
