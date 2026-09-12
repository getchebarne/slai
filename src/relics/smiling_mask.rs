use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// The shop Card-removal service always costs 50 gold
// See:
//    - `process_effect_shop_build.rs`
pub static SMILING_MASK: RelicTemplate = RelicTemplate {
    name: RelicName::SmilingMask,
    tier: RelicTier::Common,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    effects_on_rest: &[],
    effects_counter: &[],
};
