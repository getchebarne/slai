use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Unknown Rooms can no longer be Monster fights
// See:
//    - `process_effect_room_enter.rs`
pub static JUZU_BRACELET: RelicTemplate = RelicTemplate {
    name: RelicName::JuzuBracelet,
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
