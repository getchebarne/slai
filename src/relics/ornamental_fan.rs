use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 3rd Attack played each turn grants 4 block
// See:
//    - `process_effect_card_play.rs`
pub static ORNAMENTAL_FAN: RelicTemplate = RelicTemplate {
    name: RelicName::OrnamentalFan,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    effects_combat_start: &[],
};
