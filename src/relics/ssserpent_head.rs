use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_room_enter.rs` (gain 50 gold on entering a "?" room)
pub static SSSERPENT_HEAD: RelicTemplate = RelicTemplate {
    name: RelicName::SsserpentHead,
    tier: RelicTier::Special,
    counter_init: 0,
    effects_combat_start: &[],
};
