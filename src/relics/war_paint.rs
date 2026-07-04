use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, upgrade 2 random Skills
pub static WAR_PAINT: Entity =
    make_entity_relic(RelicName::WarPaint, RelicTier::Common, 0, &[]);
