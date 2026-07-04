use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Player attacks leaving a 1-4 HP remainder deal 5 instead
pub static BOOT: Entity =
    make_entity_relic(RelicName::Boot, RelicTier::Common, 0, &[]);
