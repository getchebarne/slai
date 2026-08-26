use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// At the start of turn 3 each combat, gain 18 block
// See:
//    - `process_effect_turn_start.rs`
pub static CAPTAINS_WHEEL: RelicTemplate = RelicTemplate {
    name: RelicName::CaptainsWheel,
    tier: RelicTier::Rare,
    counter_init: 0,
    effects_combat_start: &[],
};
