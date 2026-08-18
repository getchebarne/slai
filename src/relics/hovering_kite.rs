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
    effects_combat_start: &[],
};
