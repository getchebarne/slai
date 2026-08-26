use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On purchase, brews 5 Potions staged as a reward over the shop
// See:
//    - `process_effect_relic_adopt.rs`
pub static CAULDRON: RelicTemplate = RelicTemplate {
    name: RelicName::Cauldron,
    tier: RelicTier::Shop,
    counter_init: 0,
    effects_combat_start: &[],
};
