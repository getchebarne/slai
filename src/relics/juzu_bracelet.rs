use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Unknown rooms can no longer be monster fights
pub static JUZU_BRACELET: Entity =
    make_entity_relic(RelicName::JuzuBracelet, RelicTier::Common, 0, &[]);
