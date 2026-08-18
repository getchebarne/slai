use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 4th unknown room is a Treasure room
// See:
//    - `process_effect_room_enter.rs`
pub static TINY_CHEST: RelicTemplate = RelicTemplate {
    name: RelicName::TinyChest,
    tier: RelicTier::Common,
    counter_init: 0,
    effects_combat_start: &[],
};
