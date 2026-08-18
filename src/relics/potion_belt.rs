use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, gain 2 Potion slots
// See:
//    - `process_effect_relic_adopt.rs`
pub static POTION_BELT: RelicTemplate = RelicTemplate {
    name: RelicName::PotionBelt,
    tier: RelicTier::Common,
    counter_init: 0,
    effects_combat_start: &[],
};
