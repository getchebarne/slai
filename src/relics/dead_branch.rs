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
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    effects_on_rest: &[],
    effects_counter: &[],
};
