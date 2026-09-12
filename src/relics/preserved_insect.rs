use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Elite enemies start at 75 percent HP
// See:
//    - `process_effect_combat_start.rs`
pub static PRESERVED_INSECT: RelicTemplate = RelicTemplate {
    name: RelicName::PreservedInsect,
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
