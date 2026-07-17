use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Unspent energy carries over between turns
// See:
//    - `process_effect_turn_start.rs`
pub static ICE_CREAM: Entity = make_entity_relic(RelicName::IceCream, RelicTier::Rare, 0, &[]);
