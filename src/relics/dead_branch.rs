use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Exhausting a Card conjures a random Card into the hand
// See:
//    - `process_effect_card_exhaust.rs`
pub static DEAD_BRANCH: RelicTemplate = RelicTemplate {
    name: RelicName::DeadBranch,
    tier: RelicTier::Rare,
    counter_init: 0,
    effects_combat_start: &[],
};
