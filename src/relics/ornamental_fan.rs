use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 3rd Attack played each turn grants 4 block
pub static ORNAMENTAL_FAN: Entity =
    make_entity_relic(RelicName::OrnamentalFan, RelicTier::Uncommon, 0, &[]);
