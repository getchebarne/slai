use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

pub static KUNAI: Entity = make_entity_relic(RelicName::Kunai, RelicTier::Uncommon, 0, &[]);
