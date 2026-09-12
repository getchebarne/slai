use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every HP loss is reduced by 1
// See:
//    - `process_effect_health_delta.rs`
pub static TUNGSTEN_ROD: RelicTemplate = RelicTemplate {
    name: RelicName::TungstenRod,
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
