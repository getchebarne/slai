use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// The first Attack costing 2+ each turn is played twice; grants Necronomicurse on pickup
// See:
//    - `process_effect_card_play.rs`
//    - `process_effect_relic_adopt.rs`
pub static NECRONOMICON: RelicTemplate = RelicTemplate {
    name: RelicName::Necronomicon,
    tier: RelicTier::Special,
    counter_init: 0,
    effects_combat_start: &[],
};
