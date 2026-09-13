use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// After a rest site, start the next combat with 2 extra energy
// See:
//    - `process_effect_combat_start.rs`
//    - `process_effect_room_enter.rs`
pub static ANCIENT_TEA_SET: RelicTemplate = RelicTemplate {
    name: RelicName::AncientTeaSet,
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
