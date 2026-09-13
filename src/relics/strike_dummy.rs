use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Strike-tagged Cards deal 3 more damage
// See:
//    - `process_effect_damage_physical.rs`
pub static STRIKE_DUMMY: RelicTemplate = RelicTemplate {
    name: RelicName::StrikeDummy,
    tier: RelicTier::Uncommon,
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
