use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Ending the turn with 0 block grants 6 block
// See:
//    - `process_effect_turn_end.rs`
pub static ORICHALCUM: RelicTemplate = RelicTemplate {
    name: RelicName::Orichalcum,
    tier: RelicTier::Common,
    counter_init: 0,
    effects_combat_start: &[],
};
