use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 3rd turn start grants 1 energy; counter persists across combats
// See:
//    - `process_effect_turn_start.rs`
pub static HAPPY_FLOWER: RelicTemplate = RelicTemplate {
    name: RelicName::HappyFlower,
    tier: RelicTier::Common,
    counter_init: 0,
    effects_combat_start: &[],
};
