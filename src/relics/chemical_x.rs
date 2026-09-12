use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// X-cost Cards resolve with X+2; energy spent is unchanged
// See:
//    - `process_effect_card_play.rs`
pub static CHEMICAL_X: RelicTemplate = RelicTemplate {
    name: RelicName::ChemicalX,
    tier: RelicTier::Shop,
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
