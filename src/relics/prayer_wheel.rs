use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Normal Monster fights drop a second Card bundle
// See:
//    - `process_effect_reward_roll_combat.rs`
pub static PRAYER_WHEEL: RelicTemplate = RelicTemplate {
    name: RelicName::PrayerWheel,
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
