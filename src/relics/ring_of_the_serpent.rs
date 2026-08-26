use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Replaces Ring of the Snake; draw 1 additional Card each turn
// See:
//    - `process_effect_relic_adopt.rs`
//    - `process_effect_turn_start.rs`
pub static RING_OF_THE_SERPENT: RelicTemplate = RelicTemplate {
    name: RelicName::RingOfTheSerpent,
    tier: RelicTier::Boss,
    counter_init: 0,
    effects_combat_start: &[],
};
