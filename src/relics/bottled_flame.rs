use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, bottle an Attack; it starts every combat in the opening hand
// See:
//    - `process_effect_relic_adopt.rs`
//    - `process_effect_card_bottle.rs`
//    - `process_effect_combat_start.rs`
pub static BOTTLED_FLAME: RelicTemplate = RelicTemplate {
    name: RelicName::BottledFlame,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    effects_combat_start: &[],
};
