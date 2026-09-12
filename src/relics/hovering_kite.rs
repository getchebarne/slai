use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// The first Card discarded each turn grants 1 energy
// See:
//    - `process_effect_card_discard.rs`
pub static HOVERING_KITE: RelicTemplate = RelicTemplate {
    name: RelicName::HoveringKite,
    tier: RelicTier::Boss,
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
