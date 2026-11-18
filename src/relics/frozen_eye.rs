use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// No engine effect: the FFI gives every draw-pile Card its `draw_depth`
pub static FROZEN_EYE: RelicTemplate = RelicTemplate {
    name: RelicName::FrozenEye,
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
