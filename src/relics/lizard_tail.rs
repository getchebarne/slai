use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// The first lethal blow leaves the Character at half max HP instead
// See:
//    - `process_effect_death.rs`
pub static LIZARD_TAIL: RelicTemplate = RelicTemplate {
    name: RelicName::LizardTail,
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
