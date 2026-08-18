use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Ending combat at half HP or less heals 12 HP
// See:
//    - `process_effect_combat_end.rs`
pub static MEAT_ON_THE_BONE: RelicTemplate = RelicTemplate {
    name: RelicName::MeatOnTheBone,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    effects_combat_start: &[],
};
