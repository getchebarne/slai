use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Gain 6 block whenever the discard pile is shuffled into the draw pile
pub static ABACUS: Entity =
    make_entity_relic(RelicName::Abacus, RelicTier::Shop, 0, &[]);
