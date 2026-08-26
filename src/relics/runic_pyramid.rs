use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// The hand is kept at end of turn; ethereal still exhausts, retain flags still clear
// See:
//    - `process_effect_turn_end.rs`
pub static RUNIC_PYRAMID: RelicTemplate = RelicTemplate {
    name: RelicName::RunicPyramid,
    tier: RelicTier::Boss,
    counter_init: 0,
    effects_combat_start: &[],
};
