use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Dig at rest sites: a random Relic, staged as a reward the player may leave
// See:
//    - `action.rs`
//    - `process_effect_rest_dig.rs`
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
