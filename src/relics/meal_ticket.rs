use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Entering a shop heals 15 HP
// See:
//    - `process_effect_room_enter.rs`
pub static MEAL_TICKET: RelicTemplate = RelicTemplate {
    name: RelicName::MealTicket,
    tier: RelicTier::Common,
    counter_init: 0,
    effects_combat_start: &[],
};
