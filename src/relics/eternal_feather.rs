use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Entering a rest site heals 3 HP per 5 deck Cards
// See:
//    - `process_effect_room_enter.rs`
pub static ETERNAL_FEATHER: RelicTemplate = RelicTemplate {
    name: RelicName::EternalFeather,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    effects_combat_start: &[],
};
