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
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    effects_on_rest: &[],
    effects_counter: &[],
};
