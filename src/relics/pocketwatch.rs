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
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    effects_on_rest: &[],
    effects_counter: &[],
};
