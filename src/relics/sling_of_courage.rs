use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Elite combats start with 2 Strength
pub static SLING_OF_COURAGE: Entity =
    make_entity_relic(RelicName::SlingOfCourage, RelicTier::Shop, 0, &[]);
