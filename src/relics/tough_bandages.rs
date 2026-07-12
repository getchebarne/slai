use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Manually discarding a card grants 3 block
pub static TOUGH_BANDAGES: Entity =
    make_entity_relic(RelicName::ToughBandages, RelicTier::Rare, 0, &[]);
