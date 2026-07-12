use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// An enemy death mid-combat grants 1 energy and draws 1 card
pub static GREMLIN_HORN: Entity =
    make_entity_relic(RelicName::GremlinHorn, RelicTier::Uncommon, 0, &[]);
