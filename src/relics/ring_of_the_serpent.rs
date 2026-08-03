use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Replaces Ring of the Snake; draw 1 additional Card each turn
// See:
//    - `process_effect_relic_adopt.rs`
//    - `process_effect_turn_start.rs`
pub static RING_OF_THE_SERPENT: Entity =
    make_entity_relic(RelicName::RingOfTheSerpent, RelicTier::Boss, 0, &[]);
