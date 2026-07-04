use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, gain 7 max HP
pub static STRAWBERRY: Entity =
    make_entity_relic(RelicName::Strawberry, RelicTier::Common, 0, &[]);
