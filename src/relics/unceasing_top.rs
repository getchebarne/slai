use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// An empty hand during your turn draws 1 Card
// See:
//    - `engine/mod.rs`
//    - `utils.rs`
pub static UNCEASING_TOP: RelicTemplate = RelicTemplate {
    name: RelicName::UnceasingTop,
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
