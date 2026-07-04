use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Elite enemies start at 75 percent HP
pub static PRESERVED_INSECT: Entity =
    make_entity_relic(RelicName::PreservedInsect, RelicTier::Common, 0, &[]);
