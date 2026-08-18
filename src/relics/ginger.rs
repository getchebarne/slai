use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// The character can no longer become Weakened
// See:
//    - `process_effect_modifier_gain.rs`
pub static GINGER: RelicTemplate = RelicTemplate {
    name: RelicName::Ginger,
    tier: RelicTier::Rare,
    counter_init: 0,
    effects_combat_start: &[],
};
