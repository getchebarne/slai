use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_chest_open.rs` (the next chest opened is empty)
pub static NLOTHS_HUNGRY_FACE: Entity =
    make_entity_relic(RelicName::NlothsHungryFace, RelicTier::Special, 0, &[]);
