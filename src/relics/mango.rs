use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, gain 14 max HP
// See:
//    - `process_effect_relic_adopt.rs`
pub static MANGO: RelicTemplate = RelicTemplate {
    name: RelicName::Mango,
    tier: RelicTier::Rare,
    counter_init: 0,
    effects_combat_start: &[],
};
