use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Normal monster fights drop a second Card bundle
// See:
//    - `process_effect_reward_roll_combat.rs`
pub static PRAYER_WHEEL: Entity =
    make_entity_relic(RelicName::PrayerWheel, RelicTier::Rare, 0, &[]);
