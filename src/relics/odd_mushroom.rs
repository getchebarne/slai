use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `utils.rs::vuln_factor` (Vulnerable deals x1.25 to the Character instead of x1.5)
pub static ODD_MUSHROOM: RelicTemplate = RelicTemplate {
    name: RelicName::OddMushroom,
    tier: RelicTier::Special,
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
