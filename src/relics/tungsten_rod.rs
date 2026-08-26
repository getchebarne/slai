use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every HP loss is reduced by 1
// See:
//    - `process_effect_health_delta.rs`
pub static TUNGSTEN_ROD: RelicTemplate = RelicTemplate {
    name: RelicName::TungstenRod,
    tier: RelicTier::Rare,
    counter_init: 0,
    effects_combat_start: &[],
};
