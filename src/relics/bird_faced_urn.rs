use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Playing a Power heals 2 HP
pub static BIRD_FACED_URN: Entity =
    make_entity_relic(RelicName::BirdFacedUrn, RelicTier::Rare, 0, &[]);
