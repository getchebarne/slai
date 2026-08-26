use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// At turn start, lose 15 block instead of all of it
// See:
//    - `process_effect_turn_start.rs`
pub static CALIPERS: RelicTemplate = RelicTemplate {
    name: RelicName::Calipers,
    tier: RelicTier::Rare,
    counter_init: 0,
    effects_combat_start: &[],
};
