use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

pub static SHURIKEN: Entity = make_entity_relic(RelicName::Shuriken, RelicTier::Uncommon, 0, &[]);
