use crate::entity::{Entity, make_entity_relic};
use crate::types::{RelicName, RelicTier};

pub static SHURIKEN: Entity =
    make_entity_relic(RelicName::Shuriken, RelicTier::Rare, 0, &[]);
