use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Elite fights drop an additional Relic
// See:
//    - `process_effect_reward_roll_combat.rs`
pub static BLACK_STAR: RelicTemplate = RelicTemplate {
    name: RelicName::BlackStar,
    tier: RelicTier::Boss,
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
