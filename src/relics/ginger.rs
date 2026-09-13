use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// The Character can no longer become Weakened
// See:
//    - `process_effect_modifier_gain.rs`
pub static GINGER: RelicTemplate = RelicTemplate {
    name: RelicName::Ginger,
    tier: RelicTier::Rare,
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
