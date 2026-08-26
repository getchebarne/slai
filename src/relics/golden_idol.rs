use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_reward_roll_combat.rs`
pub static GOLDEN_IDOL: RelicTemplate = RelicTemplate {
    name: RelicName::GoldenIdol,
    tier: RelicTier::Special,
    counter_init: 0,
    effects_combat_start: &[],
};
