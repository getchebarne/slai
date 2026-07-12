use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// After a rest site, start the next combat with 2 extra energy
pub static ANCIENT_TEA_SET: Entity =
    make_entity_relic(RelicName::AncientTeaSet, RelicTier::Common, 0, &[]);
