use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Manually discarding a Card grants 3 block
// See:
//    - `process_effect_card_discard.rs`
pub static TOUGH_BANDAGES: RelicTemplate = RelicTemplate {
    name: RelicName::ToughBandages,
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
