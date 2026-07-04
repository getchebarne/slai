use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Incoming attacks leaving a 2-5 HP remainder deal 1 instead
pub static TORII: Entity =
    make_entity_relic(RelicName::Torii, RelicTier::Rare, 0, &[]);
