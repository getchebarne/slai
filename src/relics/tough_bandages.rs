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
    effects_combat_start: &[],
};
