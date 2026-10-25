use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// The Character can't heal, and nothing revives it; granted by Mind Bloom's I am Awake
// See:
//    - `process_effect_health_delta.rs`
//    - `process_effect_death.rs`
pub static MARK_OF_THE_BLOOM: RelicTemplate = RelicTemplate {
    name: RelicName::MarkOfTheBloom,
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
