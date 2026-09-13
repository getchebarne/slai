use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Cards that would exhaust on play are discarded instead 50% of the time
// See:
//    - `process_effect_card_play.rs`
pub static STRANGE_SPOON: RelicTemplate = RelicTemplate {
    name: RelicName::StrangeSpoon,
    tier: RelicTier::Shop,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_pickup: &[],
    effects_rest: &[],
    effects_counter: &[],
};
