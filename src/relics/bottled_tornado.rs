use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, bottle a Power; it starts every combat in the opening hand
// See:
//    - `process_effect_relic_adopt.rs`
//    - `process_effect_card_bottle.rs`
//    - `process_effect_combat_start.rs`
pub static BOTTLED_TORNADO: RelicTemplate = RelicTemplate {
    name: RelicName::BottledTornado,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    effects_combat_start: &[],
};
