use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every turn start deals 3 damage to all enemies
pub static MERCURY_HOURGLASS: Entity =
    make_entity_relic(RelicName::MercuryHourglass, RelicTier::Uncommon, 0, &[]);
