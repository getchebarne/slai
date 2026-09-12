use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// +1 energy; Rest is no longer available at rest sites
// See:
//    - `process_effect_combat_start.rs`
//    - `action.rs`
pub static COFFEE_DRIPPER: RelicTemplate = RelicTemplate {
    name: RelicName::CoffeeDripper,
    tier: RelicTier::Boss,
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
