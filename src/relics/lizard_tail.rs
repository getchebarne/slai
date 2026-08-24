use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// The first lethal blow leaves the Character at half max HP instead
// See:
//    - `process_effect_death.rs`
pub static LIZARD_TAIL: RelicTemplate = RelicTemplate {
    name: RelicName::LizardTail,
    tier: RelicTier::Rare,
    counter_init: 0,
    effects_combat_start: &[],
};
