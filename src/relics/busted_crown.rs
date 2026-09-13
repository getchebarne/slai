use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// +1 energy; combat Card rewards offer 2 fewer Cards
// See:
//    - `process_effect_combat_start.rs`
//    - `utils.rs`
pub static BUSTED_CROWN: RelicTemplate = RelicTemplate {
    name: RelicName::BustedCrown,
    tier: RelicTier::Boss,
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
