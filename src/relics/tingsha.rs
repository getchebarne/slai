use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Manually discarding a Card deals 3 damage to a random enemy
// See:
//    - `process_effect_card_discard.rs`
pub static TINGSHA: RelicTemplate = RelicTemplate {
    name: RelicName::Tingsha,
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
