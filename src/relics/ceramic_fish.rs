use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Adding a Card to the deck grants 9 gold
// See:
//    - `process_effect_card_adopt.rs`
pub static CERAMIC_FISH: RelicTemplate = RelicTemplate {
    name: RelicName::CeramicFish,
    tier: RelicTier::Common,
    counter_init: 0,
    effects_combat_start: &[],
};
