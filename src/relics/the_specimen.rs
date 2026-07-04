use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// An enemy dying with Poison passes it to a random enemy
pub static THE_SPECIMEN: Entity =
    make_entity_relic(RelicName::TheSpecimen, RelicTier::Rare, 0, &[]);
