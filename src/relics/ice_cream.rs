use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Unspent energy carries over between turns
// See:
//    - `process_effect_turn_start.rs`
pub static ICE_CREAM: RelicTemplate = RelicTemplate {
    name: RelicName::IceCream,
    tier: RelicTier::Rare,
    counter_init: 0,
    effects_combat_start: &[],
};
