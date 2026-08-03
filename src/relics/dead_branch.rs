use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Exhausting a Card conjures a random Card into the hand
// See:
//    - `process_effect_card_exhaust.rs`
pub static DEAD_BRANCH: Entity = make_entity_relic(RelicName::DeadBranch, RelicTier::Rare, 0, &[]);
