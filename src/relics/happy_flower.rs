use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 3rd turn start grants 1 energy; counter persists across combats
pub static HAPPY_FLOWER: Entity =
    make_entity_relic(RelicName::HappyFlower, RelicTier::Common, 0, &[]);
