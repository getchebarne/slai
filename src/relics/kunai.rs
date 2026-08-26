use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_card_play.rs`
pub static KUNAI: RelicTemplate = RelicTemplate {
    name: RelicName::Kunai,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    effects_combat_start: &[],
};
