use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, choose a deck card and obtain a copy of it
pub static DOLLYS_MIRROR: Entity =
    make_entity_relic(RelicName::DollysMirror, RelicTier::Shop, 0, &[]);
