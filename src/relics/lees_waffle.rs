use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, gain 7 max HP and heal to full
// See:
//    - `process_effect_relic_adopt.rs`
pub static LEES_WAFFLE: RelicTemplate = RelicTemplate {
    name: RelicName::LeesWaffle,
    tier: RelicTier::Shop,
    counter_init: 0,
    effects_combat_start: &[],
};
