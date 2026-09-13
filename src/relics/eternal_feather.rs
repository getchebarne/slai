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
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_pickup: &[],
    effects_rest: &[],
    effects_counter: &[],
};
