use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Boss combats start with a 25 HP heal
// See:
//    - `process_effect_room_enter.rs`
pub static PANTOGRAPH: RelicTemplate = RelicTemplate {
    name: RelicName::Pantograph,
    tier: RelicTier::Uncommon,
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
