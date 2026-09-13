use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, upgrade 2 random Attacks
// See:
//    - `process_effect_relic_adopt.rs`
pub static WHETSTONE: RelicTemplate = RelicTemplate {
    name: RelicName::Whetstone,
    tier: RelicTier::Common,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_pickup: &[],
    effects_rest: &[],
    effects_counter: &[],
};
