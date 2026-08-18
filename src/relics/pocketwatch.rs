use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Playing 3 or fewer Cards in a turn draws 3 extra Cards next turn
// See:
//    - `process_effect_turn_end.rs`
pub static POCKETWATCH: RelicTemplate = RelicTemplate {
    name: RelicName::Pocketwatch,
    tier: RelicTier::Rare,
    counter_init: 0,
    effects_combat_start: &[],
};
