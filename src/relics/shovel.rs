use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Dig at rest sites: gain a random Relic (granted directly, not staged as a reward)
// See:
//    - `action.rs`
pub static SHOVEL: RelicTemplate = RelicTemplate {
    name: RelicName::Shovel,
    tier: RelicTier::Rare,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_pickup: &[],
    effects_rest: &[],
    effects_counter: &[],
};
