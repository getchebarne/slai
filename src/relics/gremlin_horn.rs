use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// An enemy death mid-combat grants 1 energy and draws 1 Card
// See:
//    - `process_effect_death.rs`
pub static GREMLIN_HORN: RelicTemplate = RelicTemplate {
    name: RelicName::GremlinHorn,
    tier: RelicTier::Uncommon,
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
