use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 4th unknown Room is a Treasure Room
// See:
//    - `process_effect_room_enter.rs`
pub static TINY_CHEST: RelicTemplate = RelicTemplate {
    name: RelicName::TinyChest,
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
