use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Ending the turn with 0 block grants 6 block
// See:
//    - `process_effect_turn_end.rs`
pub static ORICHALCUM: Entity = make_entity_relic(RelicName::Orichalcum, RelicTier::Common, 0, &[]);
