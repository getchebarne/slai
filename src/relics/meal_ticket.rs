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
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    effects_on_rest: &[],
    effects_counter: &[],
};
