use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Normal monster fights drop a second card reward
// See:
//    - `process_effect_reward_roll_combat.rs`
//    - `process_effect_room_exit.rs`
pub static PRAYER_WHEEL: Entity =
    make_entity_relic(RelicName::PrayerWheel, RelicTier::Uncommon, 0, &[]);
