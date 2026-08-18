use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Normal monster fights drop a second Card bundle
// See:
//    - `process_effect_reward_roll_combat.rs`
pub static PRAYER_WHEEL: RelicTemplate = RelicTemplate {
    name: RelicName::PrayerWheel,
    tier: RelicTier::Rare,
    counter_init: 0,
    effects_combat_start: &[],
};
