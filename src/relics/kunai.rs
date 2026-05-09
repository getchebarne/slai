use crate::entity::{Entity, make_entity_relic};
use crate::types::{RelicName, RelicTier};

pub static KUNAI: Entity =
    make_entity_relic(RelicName::Kunai, RelicTier::Uncommon, 0, &[]);
