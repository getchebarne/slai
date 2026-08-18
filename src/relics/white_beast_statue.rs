use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Potions always drop after combat
// See:
//    - `process_effect_reward_roll_combat.rs`
pub static WHITE_BEAST_STATUE: RelicTemplate = RelicTemplate {
    name: RelicName::WhiteBeastStatue,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    effects_combat_start: &[],
};
