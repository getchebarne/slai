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
    effects_combat_start: &[],
};
