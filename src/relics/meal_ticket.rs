use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Entering a shop heals 15 HP
// See:
//    - `process_effect_room_enter.rs`
pub static MEAL_TICKET: Entity =
    make_entity_relic(RelicName::MealTicket, RelicTier::Common, 0, &[]);
