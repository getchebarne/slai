use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// The character can no longer become Weakened
pub static GINGER: Entity =
    make_entity_relic(RelicName::Ginger, RelicTier::Rare, 0, &[]);
