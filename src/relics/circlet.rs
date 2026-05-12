use crate::entity::{Entity, make_entity_relic};
use crate::types::{RelicName, RelicTier};

pub static CIRCLET: Entity = make_entity_relic(RelicName::Circlet, RelicTier::Special, 0, &[]);
