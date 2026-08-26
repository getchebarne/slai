use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// At combat start, choose 1 of 3 colorless Cards to add to the hand
// See:
//    - `process_effect_combat_start.rs`
//    - `process_effect_toolbox_roll.rs`
//    - `process_effect_toolbox_pick.rs`
pub static TOOLBOX: RelicTemplate = RelicTemplate {
    name: RelicName::Toolbox,
    tier: RelicTier::Shop,
    counter_init: 0,
    effects_combat_start: &[],
};
