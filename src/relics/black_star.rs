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
    effects_combat_start: &[],
};
