use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Resting heals 15 additional HP
// See:
//    - `action.rs`
pub static REGAL_PILLOW: Entity =
    make_entity_relic(RelicName::RegalPillow, RelicTier::Common, 0, &[]);
