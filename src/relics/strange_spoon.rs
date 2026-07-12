use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Cards that would exhaust on play are discarded instead 50% of the time
pub static STRANGE_SPOON: Entity =
    make_entity_relic(RelicName::StrangeSpoon, RelicTier::Shop, 0, &[]);
