use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 4th unknown room is a Treasure room
// See:
//    - `process_effect_room_enter.rs`
pub static TINY_CHEST: Entity = make_entity_relic(RelicName::TinyChest, RelicTier::Common, 0, &[]);
