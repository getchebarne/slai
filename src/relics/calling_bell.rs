use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, gain Curse of the Bell plus a Common, an Uncommon, and a Rare Relic
// See:
//    - `process_effect_relic_adopt.rs`
pub static CALLING_BELL: RelicTemplate = RelicTemplate {
    name: RelicName::CallingBell,
    tier: RelicTier::Boss,
    counter_init: 0,
    effects_combat_start: &[],
};
