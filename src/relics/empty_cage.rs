use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, remove 2 Cards from the deck
// See:
//    - `process_effect_relic_adopt.rs`
pub static EMPTY_CAGE: RelicTemplate = RelicTemplate {
    name: RelicName::EmptyCage,
    tier: RelicTier::Boss,
    counter_init: 0,
    effects_combat_start: &[],
};
