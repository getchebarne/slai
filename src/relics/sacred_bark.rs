use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Potion effects are doubled (discover Potions excluded)
// See:
//    - `process_effect_potion_use.rs`
pub static SACRED_BARK: RelicTemplate = RelicTemplate {
    name: RelicName::SacredBark,
    tier: RelicTier::Boss,
    counter_init: 0,
    effects_combat_start: &[],
};
