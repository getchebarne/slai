use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Strike-tagged cards deal 3 more damage
pub static STRIKE_DUMMY: Entity =
    make_entity_relic(RelicName::StrikeDummy, RelicTier::Uncommon, 0, &[]);
