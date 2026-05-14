use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

pub static CIRCLET: Entity = make_entity_relic(RelicName::Circlet, RelicTier::Special, 0, &[]);
