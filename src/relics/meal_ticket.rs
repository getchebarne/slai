use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Entering a shop heals 15 HP
pub static MEAL_TICKET: Entity =
    make_entity_relic(RelicName::MealTicket, RelicTier::Common, 0, &[]);
