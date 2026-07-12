use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 3rd reshuffle grants 2 energy; counter persists across combats
pub static SUNDIAL: Entity =
    make_entity_relic(RelicName::Sundial, RelicTier::Uncommon, 0, &[]);
