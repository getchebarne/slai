use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every turn start deals 3 damage to all enemies
// See:
//    - `process_effect_turn_start.rs`
pub static MERCURY_HOURGLASS: RelicTemplate = RelicTemplate {
    name: RelicName::MercuryHourglass,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    effects_combat_start: &[],
};
