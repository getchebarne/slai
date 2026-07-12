use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Exhausting a card conjures a random card into the hand
// See:
//    - `process_effect_card_exhaust.rs`
pub static DEAD_BRANCH: Entity =
    make_entity_relic(RelicName::DeadBranch, RelicTier::Rare, 0, &[]);
