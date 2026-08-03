use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Entering any room grants 12 gold until gold is spent at a shop
// See:
//    - `process_effect_gold_delta.rs`
//    - `process_effect_room_enter.rs`
pub static MAW_BANK: Entity = make_entity_relic(RelicName::MawBank, RelicTier::Common, 0, &[]);
