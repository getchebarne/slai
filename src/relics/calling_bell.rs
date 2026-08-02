use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, gain Curse of the Bell plus a Common, an Uncommon, and a Rare Relic
// See:
//    - `process_effect_relic_adopt.rs`
pub static CALLING_BELL: Entity =
    make_entity_relic(RelicName::CallingBell, RelicTier::Boss, 0, &[]);
