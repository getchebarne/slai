use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_chest_open.rs` (the next chest opened is empty)
pub static NLOTHS_HUNGRY_FACE: RelicTemplate = RelicTemplate {
    name: RelicName::NlothsHungryFace,
    tier: RelicTier::Special,
    counter_init: 0,
    effects_combat_start: &[],
};
