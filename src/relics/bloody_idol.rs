use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Heal 5 HP whenever gold is gained
// See:
//    - `process_effect_gold_delta.rs`
pub static BLOODY_IDOL: RelicTemplate = RelicTemplate {
    name: RelicName::BloodyIdol,
    tier: RelicTier::Special,
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
