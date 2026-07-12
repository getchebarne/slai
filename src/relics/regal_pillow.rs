use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Resting heals 15 additional HP
// See:
//    - `action.rs`
pub static REGAL_PILLOW: Entity =
    make_entity_relic(RelicName::RegalPillow, RelicTier::Common, 0, &[],
    "Heal an additional 15 HP when you Rest.",
);
