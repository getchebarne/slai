use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// At turn start, lose 15 block instead of all of it
// See:
//    - `process_effect_turn_start.rs`
pub static CALIPERS: Entity = make_entity_relic(RelicName::Calipers, RelicTier::Rare, 0, &[]);
