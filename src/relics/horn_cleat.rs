use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// At the start of turn 2 each combat, gain 14 block
// See:
//    - `process_effect_turn_start.rs`
pub static HORN_CLEAT: RelicTemplate = RelicTemplate {
    name: RelicName::HornCleat,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    effects_combat_start: &[],
};
