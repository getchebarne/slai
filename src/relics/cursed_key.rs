use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// +1 energy; opening a chest adds a random Curse to the deck
// See:
//    - `process_effect_combat_start.rs`
//    - `process_effect_chest_open.rs`
pub static CURSED_KEY: RelicTemplate = RelicTemplate {
    name: RelicName::CursedKey,
    tier: RelicTier::Boss,
    counter_init: 0,
    effects_combat_start: &[],
};
